const { loadVdfGmp } = require('./index.js');

async function main() {
  const vdf = await loadVdfGmp();
  const challenge = Buffer.from([0xaa]);
  const difficulty = 500n;
  const proof = vdf.solveWesolowski(challenge, difficulty, 40);
  if (!vdf.verifyWesolowski(challenge, difficulty, proof, 40)) {
    throw new Error('Wesolowski proof did not verify');
  }
  console.log('gmp wesolowski ok', {
    proofBytes: proof.length,
    proofHex: proof.toString('hex'),
  });
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
