export declare interface NodeRepr {
  /** The node object, cann't be instantiated in javascript. So call the constructor will throw an error */
  constructor(): void;
  select(selectors: "html"): NodeRepr;
  select(selectors: "head"): NodeRepr;
  select(selectors: "body"): NodeRepr;
}
