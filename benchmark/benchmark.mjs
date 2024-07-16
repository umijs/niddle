import cheerio from "./cheerio.mjs";
import niddle from "./niddle.mjs";
import cheerioHtmlParser2 from "./cheerio-htmlparser2.mjs";

async function main() {
  await niddle();
  await cheerio();
  await cheerioHtmlParser2();
}

main();
