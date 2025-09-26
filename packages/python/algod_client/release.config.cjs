const releaseUtils = require("../../../utils/semantic-release.cjs");

module.exports = releaseUtils.getConfig({
  language: "python",
  packageName: "algod_client",
  assets: ["../../../artifacts/*.whl"],
});
