// Bundler entry point — re-exports sigma_bridge functions as window globals
// so wasm-bindgen can call them without ES module resolution.
import {
  initSigma,
  loadGraphData,
  highlightPrereqChain,
  navigateToNode,
  clearSelection,
  killSigma,
} from "./sigma_bridge.js";

window.__sigma_bridge = {
  initSigma,
  loadGraphData,
  highlightPrereqChain,
  navigateToNode,
  clearSelection,
  killSigma,
};
