const createVdfGmpModule = require('./pkg/vdf-wasm-gmp.js');

function splitU64(value) {
  const n = BigInt(value);
  return [Number(n & 0xffffffffn), Number(n >> 32n)];
}

function readBytes(module, ptr, len) {
  return Buffer.from(module.HEAPU8.slice(ptr, ptr + len));
}

function readLastResult(module) {
  return readBytes(module, module._vdf_last_result_ptr(), module._vdf_last_result_len());
}

function readLastError(module) {
  return readBytes(module, module._vdf_last_error_ptr(), module._vdf_last_error_len()).toString(
    'utf8',
  );
}

function withBytes(module, bytes, fn) {
  const buffer = Buffer.from(bytes);
  const ptr = module._vdf_alloc(buffer.length);
  module.HEAPU8.set(buffer, ptr);
  try {
    return fn(ptr, buffer.length);
  } finally {
    module._vdf_dealloc(ptr, buffer.length);
  }
}

function assertOk(module, code) {
  if (code < 0) {
    throw new Error(readLastError(module) || `VDF call failed with code ${code}`);
  }
}

function solve(module, fn, challenge, difficulty, intSizeBits) {
  const [lo, hi] = splitU64(difficulty);
  return withBytes(module, challenge, (challengePtr, challengeLen) => {
    const code = fn(challengePtr, challengeLen, lo, hi, intSizeBits);
    assertOk(module, code);
    return readLastResult(module);
  });
}

function verify(module, fn, challenge, difficulty, proof, intSizeBits) {
  const [lo, hi] = splitU64(difficulty);
  return withBytes(module, challenge, (challengePtr, challengeLen) =>
    withBytes(module, proof, (proofPtr, proofLen) => {
      const code = fn(challengePtr, challengeLen, lo, hi, proofPtr, proofLen, intSizeBits);
      assertOk(module, code);
      return code === 1;
    }),
  );
}

async function loadVdfGmp(options = {}) {
  const module = await createVdfGmpModule({ noInitialRun: true, ...options });
  return {
    module,
    solveWesolowski(challenge, difficulty, intSizeBits) {
      return solve(module, module._vdf_solve_wesolowski, challenge, difficulty, intSizeBits);
    },
    verifyWesolowski(challenge, difficulty, proof, intSizeBits) {
      return verify(
        module,
        module._vdf_verify_wesolowski,
        challenge,
        difficulty,
        proof,
        intSizeBits,
      );
    },
  };
}

module.exports = { loadVdfGmp };
