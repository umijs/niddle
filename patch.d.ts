export declare interface NodeRepr {
  select(selectors: "html"): NodeRepr;
  select(selectors: "head"): NodeRepr;
  select(selectors: "body"): NodeRepr;
}
