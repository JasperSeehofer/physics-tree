import Sigma from "sigma";
import Graph from "graphology";
import FA2Layout from "graphology-layout-forceatlas2/worker";
import { fitViewportToNodes } from "@sigma/utils";

// Module-level state (lives in JS-land, never crosses WASM boundary per frame)
let sigmaInstance = null;
let graphInstance = null;
let fa2Worker = null;
let selectedNodeId = null;
let prereqChainSet = new Set();
let userProgressMap = {};  // {nodeId: xpAmount} — populated by updateUserProgress()
let overdueMap = {};       // {nodeId: daysOverdue} — populated by updateOverdueMap()

// Color tokens from style/main.css botanical design system
const COLORS = {
  void: "#0d0f14",
  barkDark: "#1a1d24",
  barkMid: "#252932",
  barkLight: "#2e3340",
  leafGreen: "#4caf7d",
  sunAmber: "#f4b942",
  nebulaPurple: "#8b5cf6",
  petalWhite: "#f0f2f5",
  mist: "#8892a4",
};

// Depth tier -> node visual properties (from UI-SPEC)
const DEPTH_TIER_STYLES = {
  root: { color: COLORS.barkLight, size: 12, borderColor: COLORS.nebulaPurple },
  trunk: { color: COLORS.barkMid, size: 10, borderColor: COLORS.petalWhite },
  branch: { color: COLORS.barkMid, size: 8, borderColor: COLORS.leafGreen },
  leaf: { color: hexWithAlpha(COLORS.leafGreen, 0.7), size: 6, borderColor: COLORS.petalWhite },
};

function hexWithAlpha(hex, alpha) {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgba(${r},${g},${b},${alpha})`;
}

// Edge type -> visual style per CONTEXT.md locked decision:
//   prerequisite = solid, derives_from = dashed,
//   applies_to = dotted, mathematical_foundation = double line.
//
// Sigma 3.x WebGL renders all edges as solid lines. To achieve
// dashed/dotted/double styles, we use a two-layer approach:
//   1. EDGE_TYPE_STYLES defines distinct color and width per type
//      so edges are visually distinguishable even in pure WebGL.
//   2. drawEdgeOverlay() draws canvas-based dash/dot/double patterns
//      on a second canvas layered over the WebGL canvas, using
//      Sigma's afterRender event + graphToViewport coordinate conversion.
//
// This ensures the locked decision is honored: each EdgeType has a
// visually distinct line style, not just color variation.
const EDGE_TYPE_STYLES = {
  Prerequisite:          { color: COLORS.barkLight,    size: 1.5, lineStyle: "solid" },
  DerivesFrom:           { color: COLORS.mist,         size: 1.5, lineStyle: "dashed" },
  AppliesTo:             { color: COLORS.barkLight,    size: 1.0, lineStyle: "dotted" },
  MathematicalFoundation:{ color: COLORS.nebulaPurple, size: 2.0, lineStyle: "double" },
};

// Canvas overlay for non-solid edge styles (dashed, dotted, double)
// Called after each Sigma render frame via the "afterRender" event.
function drawEdgeOverlay() {
  if (!sigmaInstance || !graphInstance) return;

  const overlayCanvas = sigmaInstance.getCanvases().edgeLabels;
  if (!overlayCanvas) return;
  const ctx = overlayCanvas.getContext("2d");
  if (!ctx) return;

  ctx.clearRect(0, 0, overlayCanvas.width, overlayCanvas.height);

  // Only draw overlay for non-solid edges
  graphInstance.forEachEdge((edge, attrs, source, target) => {
    const style = EDGE_TYPE_STYLES[attrs.edgeType] || EDGE_TYPE_STYLES.Prerequisite;
    if (style.lineStyle === "solid") return; // WebGL handles solid edges fine

    // Get screen coordinates from Sigma's viewport conversion
    const sourcePos = sigmaInstance.graphToViewport(
      graphInstance.getNodeAttributes(source)
    );
    const targetPos = sigmaInstance.graphToViewport(
      graphInstance.getNodeAttributes(target)
    );

    // Determine edge color (respect reducer dimming/highlighting)
    let edgeColor = style.color;
    if (selectedNodeId !== null) {
      const isHighlighted =
        (prereqChainSet.has(source) || source === selectedNodeId) &&
        (prereqChainSet.has(target) || target === selectedNodeId);
      if (isHighlighted) {
        edgeColor = COLORS.sunAmber;
      } else {
        edgeColor = hexWithAlpha(style.color, 0.15);
      }
    } else {
      edgeColor = hexWithAlpha(style.color, 0.6);
    }

    ctx.strokeStyle = edgeColor;
    ctx.lineWidth = style.size * (window.devicePixelRatio || 1);

    if (style.lineStyle === "dashed") {
      // 4px dash, 4px gap per UI-SPEC
      ctx.setLineDash([4, 4]);
      ctx.beginPath();
      ctx.moveTo(sourcePos.x, sourcePos.y);
      ctx.lineTo(targetPos.x, targetPos.y);
      ctx.stroke();
      ctx.setLineDash([]);
    } else if (style.lineStyle === "dotted") {
      // 2px dot, 4px gap per UI-SPEC
      ctx.setLineDash([2, 4]);
      ctx.beginPath();
      ctx.moveTo(sourcePos.x, sourcePos.y);
      ctx.lineTo(targetPos.x, targetPos.y);
      ctx.stroke();
      ctx.setLineDash([]);
    } else if (style.lineStyle === "double") {
      // Two parallel 1px lines, 3px gap per UI-SPEC
      const dx = targetPos.x - sourcePos.x;
      const dy = targetPos.y - sourcePos.y;
      const len = Math.sqrt(dx * dx + dy * dy);
      if (len === 0) return;
      const nx = -dy / len * 1.5; // perpendicular offset (half of 3px gap)
      const ny = dx / len * 1.5;

      ctx.setLineDash([]);
      ctx.lineWidth = 1 * (window.devicePixelRatio || 1);
      ctx.beginPath();
      ctx.moveTo(sourcePos.x + nx, sourcePos.y + ny);
      ctx.lineTo(targetPos.x + nx, targetPos.y + ny);
      ctx.stroke();
      ctx.beginPath();
      ctx.moveTo(sourcePos.x - nx, sourcePos.y - ny);
      ctx.lineTo(targetPos.x - nx, targetPos.y - ny);
      ctx.stroke();
    }
  });
}

// ── Wilting helper ────────────────────────────────────────────────────────
// Called in botanicalNodeReducer AFTER growth-stage styling.
// Degrades color/size per overdue severity (D-09) without changing mastery tier shape.
// Pre-computed overdueMap lookup: O(1) per node, zero per-frame computation.
function applyWiltingStyle(res, daysOverdue) {
  if (daysOverdue >= 7) {
    // Severe: gray/wilted — strongest visual signal per D-09
    res.color = COLORS.mist;
    res.size = (res.size || 8) * 0.8;
  } else if (daysOverdue >= 4) {
    // Moderate: desaturated with slight shrink per D-09
    res.color = hexWithAlpha(COLORS.mist, 0.7);
    res.size = (res.size || 8) * 0.9;
  } else if (daysOverdue >= 1) {
    // Mild: slightly faded per D-09
    res.color = hexWithAlpha(res.color, 0.6);
  }
  // < 1 day (current): no change
}

// ── User progress helpers ──────────────────────────────────────────────────

function isFrontierNode(nodeId) {
  if (!graphInstance || !graphInstance.hasNode(nodeId)) return false;
  return graphInstance.neighbors(nodeId).some(n => (userProgressMap[n] ?? 0) > 0);
}

function applyGrowthStageStyle(res, xp) {
  if (xp >= 300) {
    // Gold / bloom
    res.color = COLORS.leafGreen;
    res.size = (res.size || 8) * 1.2;
  } else if (xp >= 150) {
    // Silver / leaf
    res.color = COLORS.mist;
    res.size = (res.size || 8) * 1.0;
  } else if (xp >= 50) {
    // Bronze / sprout
    res.color = COLORS.sunAmber;
    res.size = (res.size || 8) * 0.9;
  } else {
    // Seed / dormant (1-49 XP)
    res.color = COLORS.barkLight;
    res.size = (res.size || 8) * 0.75;
  }
}

// ── Botanical canvas overlay shapes ───────────────────────────────────────

function drawBloom(ctx, x, y, size) {
  // 6-petal flower with glow
  ctx.save();
  ctx.shadowColor = COLORS.leafGreen;
  ctx.shadowBlur = 8;
  ctx.fillStyle = COLORS.leafGreen;

  // Center circle
  ctx.beginPath();
  ctx.arc(x, y, size * 0.4, 0, Math.PI * 2);
  ctx.fill();

  // 6 petals at 60-degree intervals
  for (let i = 0; i < 6; i++) {
    const angle = (Math.PI * 2 * i) / 6;
    const px = x + Math.cos(angle) * size * 0.7;
    const py = y + Math.sin(angle) * size * 0.7;
    ctx.beginPath();
    ctx.arc(px, py, size * 0.25, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.restore();
}

function drawLeaf(ctx, x, y, size) {
  // 4-point diamond (leaf shape)
  ctx.fillStyle = COLORS.mist;
  ctx.beginPath();
  ctx.moveTo(x, y - size * 0.8);
  ctx.lineTo(x + size * 0.6, y);
  ctx.lineTo(x, y + size * 0.8);
  ctx.lineTo(x - size * 0.6, y);
  ctx.closePath();
  ctx.fill();
}

function drawSprout(ctx, x, y, size) {
  // Circle with 3 upward petal stubs
  ctx.fillStyle = COLORS.sunAmber;
  ctx.globalAlpha = 0.8;
  ctx.beginPath();
  ctx.arc(x, y, size * 0.5, 0, Math.PI * 2);
  ctx.fill();
  ctx.globalAlpha = 1.0;

  // 3 petal stubs extending upward
  ctx.strokeStyle = COLORS.sunAmber;
  ctx.lineWidth = 2;
  ctx.lineCap = "round";
  const stubLen = size * 0.6;
  // Left stub
  ctx.beginPath();
  ctx.moveTo(x, y - size * 0.5);
  ctx.lineTo(x - size * 0.3, y - size * 0.5 - stubLen);
  ctx.stroke();
  // Center stub
  ctx.beginPath();
  ctx.moveTo(x, y - size * 0.5);
  ctx.lineTo(x, y - size * 0.5 - stubLen * 1.2);
  ctx.stroke();
  // Right stub
  ctx.beginPath();
  ctx.moveTo(x, y - size * 0.5);
  ctx.lineTo(x + size * 0.3, y - size * 0.5 - stubLen);
  ctx.stroke();
}

function drawBotanicalNodeOverlay() {
  if (!sigmaInstance || !graphInstance || Object.keys(userProgressMap).length === 0) return;

  const overlayCanvas = sigmaInstance.getCanvases().edgeLabels;
  if (!overlayCanvas) return;
  const octx = overlayCanvas.getContext("2d");
  if (!octx) return;

  // NOTE: drawEdgeOverlay already clears edgeLabels canvas and draws edges.
  // drawBotanicalNodeOverlay draws AFTER edges, so botanical shapes layer above
  // edge overlays (correct z-order). Do NOT clearRect here.

  // Draw botanical growth shapes for nodes with progress
  Object.entries(userProgressMap).forEach(([nodeId, xp]) => {
    if (!graphInstance.hasNode(nodeId)) return;
    if (xp <= 0) return;

    const nodeAttrs = graphInstance.getNodeAttributes(nodeId);
    if (nodeAttrs.hidden) return;

    const pos = sigmaInstance.graphToViewport(nodeAttrs);
    const size = sigmaInstance.getNodeDisplayData(nodeId)?.size || 8;
    const scaledSize = size * (window.devicePixelRatio || 1);

    // Apply wilting alpha to botanical canvas shapes — mirrors node color treatment
    const nodeDaysOverdue = overdueMap[nodeId];
    let wiltAlpha = 1.0;
    if (nodeDaysOverdue !== undefined) {
      if (nodeDaysOverdue >= 7) {
        wiltAlpha = 0.4;
      } else if (nodeDaysOverdue >= 4) {
        wiltAlpha = 0.6;
      } else if (nodeDaysOverdue >= 1) {
        wiltAlpha = 0.75;
      }
    }

    octx.save();
    octx.globalAlpha = wiltAlpha;

    if (xp >= 300) {
      drawBloom(octx, pos.x, pos.y, scaledSize);
    } else if (xp >= 150) {
      drawLeaf(octx, pos.x, pos.y, scaledSize);
    } else if (xp >= 50) {
      drawSprout(octx, pos.x, pos.y, scaledSize);
    }
    // Seeds (< 50 XP) have no special overlay — just the dim circle from the reducer

    octx.restore();
  });
}

function botanicalNodeReducer(node, data) {
  const res = { ...data };

  // Growth stage styling: show botanical overlays for learned concepts
  // All nodes remain visible — the main graph is always a full exploratory view.
  // Progressive reveal (hiding non-learned nodes) belongs on the dashboard MiniTree only.
  if (Object.keys(userProgressMap).length > 0) {
    const nodeXp = userProgressMap[node];

    // Apply growth stage styling to learned nodes
    if (nodeXp !== undefined && nodeXp > 0) {
      applyGrowthStageStyle(res, nodeXp);
    }

    // Update tooltip/label for nodes with progress
    if (nodeXp !== undefined) {
      if (nodeXp >= 300) {
        res.label = `${data.label} \u2014 Gold \u00b7 Mastered`;
      } else if (nodeXp >= 150) {
        res.label = `${data.label} \u2014 Silver \u00b7 ${nodeXp} XP`;
      } else if (nodeXp >= 50) {
        res.label = `${data.label} \u2014 Bronze \u00b7 ${nodeXp} XP`;
      } else if (nodeXp > 0) {
        res.label = `${data.label} \u2014 ${nodeXp} XP`;
      }
    }
  }

  // Wilting: apply AFTER growth stage so mastery shape is preserved (D-09)
  // overdueMap is pre-computed module-level state — no per-frame computation
  const daysOverdue = overdueMap[node];
  if (daysOverdue !== undefined && daysOverdue >= 1) {
    applyWiltingStyle(res, daysOverdue);
  }

  if (selectedNodeId === null) return res;

  if (node === selectedNodeId) {
    res.color = COLORS.leafGreen;
    res.highlighted = true;
    res.size = (data.size || 8) * 1.3;
    res.zIndex = 2;
    return res;
  }

  if (prereqChainSet.has(node)) {
    res.color = COLORS.sunAmber;
    res.zIndex = 1;
    return res;
  }

  // Dimmed: 30% opacity per CONTEXT.md decision
  res.color = hexWithAlpha(data.color || COLORS.barkMid, 0.3);
  res.labelColor = hexWithAlpha(COLORS.petalWhite, 0.3);
  res.zIndex = 0;
  return res;
}

function botanicalEdgeReducer(edge, data) {
  const res = { ...data };
  const style = EDGE_TYPE_STYLES[data.edgeType] || EDGE_TYPE_STYLES.Prerequisite;

  // For non-solid edges, hide the WebGL line (canvas overlay draws them instead)
  if (style.lineStyle !== "solid") {
    res.hidden = true;
    return res;
  }

  if (selectedNodeId === null) {
    res.color = hexWithAlpha(data.color || COLORS.barkLight, 0.6);
    return res;
  }

  const graph = graphInstance;
  const source = graph.source(edge);
  const target = graph.target(edge);
  const isHighlighted =
    (prereqChainSet.has(source) || source === selectedNodeId) &&
    (prereqChainSet.has(target) || target === selectedNodeId);

  if (isHighlighted) {
    res.color = COLORS.sunAmber;
    res.size = 3;
    res.zIndex = 1;
    return res;
  }

  // Dimmed edges: 15% opacity per UI-SPEC
  res.color = hexWithAlpha(data.color || COLORS.barkLight, 0.15);
  res.zIndex = 0;
  return res;
}

// Called from Rust via wasm-bindgen after the canvas div is mounted
export function initSigma(container, onNodeClick, onNodeEnter, onNodeLeave) {
  graphInstance = new Graph();
  sigmaInstance = new Sigma(graphInstance, container, {
    renderEdgeLabels: false,
    allowInvalidContainer: true,
    labelColor: { color: COLORS.petalWhite },
    labelFont: "Nunito",
    labelSize: 14,
    labelWeight: "700",
    stagePadding: 40,
    nodeReducer: (node, data) => botanicalNodeReducer(node, data),
    edgeReducer: (edge, data) => botanicalEdgeReducer(edge, data),
  });

  // Register canvas overlay for dashed/dotted/double edge rendering and botanical node shapes
  sigmaInstance.on("afterRender", () => {
    drawEdgeOverlay();
    drawBotanicalNodeOverlay();
  });

  sigmaInstance.on("clickNode", ({ node }) => {
    onNodeClick(node);
  });

  sigmaInstance.on("enterNode", ({ node }) => {
    onNodeEnter(node);
  });

  sigmaInstance.on("leaveNode", ({ node }) => {
    onNodeLeave(node);
  });

  // Click on empty canvas = deselect
  sigmaInstance.on("clickStage", () => {
    onNodeClick(""); // Empty string signals deselection to Rust
  });
}

// Update user progress map and refresh Sigma rendering (called from Rust via bridge)
export function updateUserProgress(progressJson) {
  userProgressMap = progressJson ? JSON.parse(progressJson) : {};
  if (sigmaInstance) sigmaInstance.refresh();
}

// Update overdue map and refresh Sigma rendering (called from Rust via bridge after review queue fetch)
// overdueJson is a JSON object mapping nodeId -> daysOverdue (e.g. {"uuid1": 3.5, "uuid2": 8.1})
export function updateOverdueMap(overdueJson) {
  overdueMap = overdueJson ? JSON.parse(overdueJson) : {};
  if (sigmaInstance) sigmaInstance.refresh();
}

// Load graph data from JSON strings (called from Rust after API fetch)
export function loadGraphData(nodesJson, edgesJson) {
  if (!graphInstance) return;

  const nodes = JSON.parse(nodesJson);
  const edges = JSON.parse(edgesJson);

  // Stage 1: Add nodes with hierarchical positioning
  const depthYMap = { root: 0.0, trunk: 0.3, branch: 0.6, leaf: 1.0 };
  const branchGroups = {};

  nodes.forEach((node) => {
    const branch = node.branch || "unknown";
    if (!branchGroups[branch]) branchGroups[branch] = [];
    branchGroups[branch].push(node);
  });

  // Spread branches horizontally
  let branchX = 0;
  const branchSpacing = 3.0;

  for (const [branch, branchNodes] of Object.entries(branchGroups)) {
    branchNodes.forEach((node, i) => {
      const tier = node.depth_tier || "branch";
      const style = DEPTH_TIER_STYLES[tier] || DEPTH_TIER_STYLES.branch;
      const y = -(depthYMap[tier] ?? 0.5) * 5; // Negative y = roots at bottom, leaves at top

      graphInstance.addNode(node.id, {
        label: node.title,
        x: branchX + (i % 5) * 0.5 + Math.random() * 0.2,
        y: y + Math.random() * 0.3,
        size: style.size,
        color: style.color,
        borderColor: style.borderColor,
        // Store metadata for reducers and panel
        nodeType: node.node_type,
        branch: node.branch,
        depthTier: node.depth_tier,
        slug: node.slug,
        description: node.description || "",
      });
    });
    branchX += branchSpacing;
  }

  // Stage 2: Add edges with type-based styling
  edges.forEach((edge) => {
    const edgeKey = `${edge.from_node}-${edge.to_node}-${edge.edge_type}`;
    if (graphInstance.hasNode(edge.from_node) && graphInstance.hasNode(edge.to_node)) {
      const style = EDGE_TYPE_STYLES[edge.edge_type] || EDGE_TYPE_STYLES.Prerequisite;
      graphInstance.addEdgeWithKey(edgeKey, edge.from_node, edge.to_node, {
        color: style.color,
        size: style.size,
        edgeType: edge.edge_type,
        lineStyle: style.lineStyle,
        weight: edge.weight,
      });
    }
  });

  // Stage 3: FA2 worker refines inter-branch spacing
  fa2Worker = new FA2Layout(graphInstance, {
    settings: {
      gravity: 1,
      scalingRatio: 10,
      barnesHutOptimize: true,
      strongGravityMode: false,
      slowDown: 5,
    },
  });
  fa2Worker.start();

  // Stop FA2 after 3 seconds (convergence timeout)
  setTimeout(() => {
    if (fa2Worker) {
      fa2Worker.stop();
      fa2Worker.kill();
      fa2Worker = null;
    }
  }, 3000);

  // Fit viewport to show entire grove
  if (sigmaInstance) {
    setTimeout(() => {
      const camera = sigmaInstance.getCamera();
      camera.animate({ x: 0.5, y: 0.5, ratio: 1 }, { duration: 0 });
    }, 100);
  }
}

// Highlight prerequisite chain for selected node
export function highlightPrereqChain(nodeId, prereqIdsJson) {
  selectedNodeId = nodeId || null;
  prereqChainSet = new Set();

  if (nodeId && prereqIdsJson) {
    const ids = JSON.parse(prereqIdsJson);
    ids.forEach((id) => prereqChainSet.add(id));
  }

  if (sigmaInstance) sigmaInstance.refresh();
}

// Navigate camera to a specific node
export function navigateToNode(nodeId) {
  if (!sigmaInstance || !graphInstance) return;
  if (!graphInstance.hasNode(nodeId)) return;

  fitViewportToNodes(sigmaInstance, [nodeId], {
    animate: true,
    duration: 500,
  });
}

// Clear current selection (deselect)
export function clearSelection() {
  selectedNodeId = null;
  prereqChainSet = new Set();
  if (sigmaInstance) sigmaInstance.refresh();
}

// Clean up everything (called from Leptos on_cleanup)
export function killSigma() {
  if (fa2Worker) {
    fa2Worker.stop();
    fa2Worker.kill();
    fa2Worker = null;
  }
  if (sigmaInstance) {
    sigmaInstance.kill();
    sigmaInstance = null;
  }
  graphInstance = null;
  selectedNodeId = null;
  prereqChainSet = new Set();
  userProgressMap = {};
  overdueMap = {};
}
