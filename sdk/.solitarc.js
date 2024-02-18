// @ts-check
const path = require('path');
const programDir = path.join(__dirname, '../auction-house-v2/programs/auction-house-v2');
const idlDir = path.join(__dirname, './idl');
const sdkDir = path.join(__dirname, 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'auction_house_v2',
  programId: 'AHV2XGm1jVAZp3NtwdVyHkbskbxk3oMfn73SXBkejUQb',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};