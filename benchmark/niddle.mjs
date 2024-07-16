import * as niddle from "../index.js";
import benchmark from "htmlparser-benchmark";

export default function cheerio() {
  return new Promise((res) => {
    var bench = benchmark(function (html, callback) {
      const $ = niddle.parse(html);
      $.outerHtml();
      callback();
    });

    bench.on("progress", function (key) {
      // console.log('finished parsing ' + key + '.html');
    });

    bench.on("result", function (stat) {
      console.log(
        "niddle         :" +
          stat.mean().toPrecision(6) +
          " ms/file ± " +
          stat.sd().toPrecision(6),
      );
      res();
    });
  });
}
