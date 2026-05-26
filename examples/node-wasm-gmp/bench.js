const { loadVdfGmp } = require('./index.js');

function timeMs(fn, repeats) {
  const start = process.hrtime.bigint();
  let value;
  for (let i = 0; i < repeats; i += 1) {
    value = fn();
  }
  const elapsed = Number(process.hrtime.bigint() - start) / 1e6;
  return [elapsed / repeats, value];
}

async function main() {
  const vdf = await loadVdfGmp();
  const challenge = Buffer.from([0xaa]);
  const bits = Number(process.env.VDF_BITS || 256);
  const wesolowskiDifficulty = BigInt(process.env.VDF_WESOLOWSKI_DIFFICULTY || 500);
  const repeats = Number(process.env.VDF_REPEATS || 5);

  const [solveMs, proof] = timeMs(
    () => vdf.solveWesolowski(challenge, wesolowskiDifficulty, bits),
    repeats,
  );
  const [verifyMs] = timeMs(() => {
    if (!vdf.verifyWesolowski(challenge, wesolowskiDifficulty, proof, bits)) {
      throw new Error('wesolowski proof did not verify');
    }
  }, repeats);

  console.log(
    `wesolowski bits=${bits} difficulty=${wesolowskiDifficulty} repeats=${repeats} solve_ms=${solveMs.toFixed(
      3,
    )} verify_ms=${verifyMs.toFixed(3)} proof_bytes=${proof.length}`,
  );
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
