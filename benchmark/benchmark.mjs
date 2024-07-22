import niddle from "./niddle.mjs";
import cheerioParse5 from "./cheerio-parse5.mjs";
import cheerioHtmlParser2 from "./cheerio-htmlparser2.mjs";

async function main() {
  await niddle();
  await cheerioParse5();
  await cheerioHtmlParser2();
}

main();
