import cheerio from "./cheerio.mjs";
import niddle from "./niddle.mjs";

async function main() {
  await niddle();
  await cheerio();
}

main();
