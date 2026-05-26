const { loadVdfGmp } = require('./index.js');

async function main() {
  const vdf = await loadVdfGmp();
  const challenge = Buffer.from([0xaa]);

  const wesolowskiDifficulty = 20n;
  const wesolowskiProof = vdf.solveWesolowski(challenge, wesolowskiDifficulty, 40);
  if (!vdf.verifyWesolowski(challenge, wesolowskiDifficulty, wesolowskiProof, 40)) {
    throw new Error('Wesolowski proof did not verify');
  }
  console.log('gmp wesolowski ok', {
    proofBytes: wesolowskiProof.length,
    proofHex: wesolowskiProof.toString('hex'),
  });

}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
