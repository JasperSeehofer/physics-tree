/**
 * mathjs_bridge.js — Symbolic formula equivalence checker using math.js.
 *
 * Loaded lazily on concept pages that contain formula quiz questions.
 * Exposes `window.__mathjs_bridge` with a `checkEquivalence` function that
 * validates user-entered LaTeX expressions against expected answers by
 * sampling at random variable values (per CONTEXT.md D-21).
 */

import { parse } from 'mathjs';

window.__mathjs_bridge = {
    /**
     * Check whether `userExpr` is symbolically equivalent to `expectedExpr`
     * by sampling `variables` at random points and comparing evaluated values.
     *
     * @param {string} userExpr     - User's expression (plain math notation, e.g. "v*t - 0.5*g*t^2")
     * @param {string} expectedExpr - Expected expression (same notation)
     * @param {string} variablesJson - JSON array of variable names, e.g. '["v","t","g"]'
     * @returns {boolean} true if expressions agree at all sample points
     */
    checkEquivalence(userExpr, expectedExpr, variablesJson) {
        try {
            const variables = JSON.parse(variablesJson);
            const userParsed = parse(userExpr);
            const expectedParsed = parse(expectedExpr);
            const samplePoints = 5;

            for (let i = 0; i < samplePoints; i++) {
                const scope = {};
                for (const v of variables) {
                    // Random value in (0.1, 10.1) — avoids zero for division safety
                    scope[v] = Math.random() * 10 + 0.1;
                }
                const userVal = userParsed.evaluate(scope);
                const expectedVal = expectedParsed.evaluate(scope);

                // Handle complex numbers (math.js may return {re, im})
                const userNum = (typeof userVal === 'object' && 're' in userVal) ? userVal.re : Number(userVal);
                const expNum = (typeof expectedVal === 'object' && 're' in expectedVal) ? expectedVal.re : Number(expectedVal);

                if (!isFinite(userNum) || !isFinite(expNum)) return false;
                if (Math.abs(userNum - expNum) > 1e-6 * (Math.abs(expNum) + 1)) return false;
            }
            return true;
        } catch (e) {
            // Parse error or eval error — treat as wrong
            return false;
        }
    },
};
