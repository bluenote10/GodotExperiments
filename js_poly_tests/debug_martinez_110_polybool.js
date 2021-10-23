#!/usr/bin/env node
// Test martinez bug https://github.com/w8r/martinez/issues/110 with Polybooljs

const converter = require("./converter");

var bug_id = 110;

var PolyBool = require('polybooljs');

let polygon1 = {
  "regions":[
    [[115,96], [140,206], [120,210], [125,250], [80,300]]
  ],"inverted":false
}
let polygon2 = {
  "regions":[
    [[111,228], [129,192], [309,282]]
  ],"inverted":false
}

converter.store_polygons([polygon1, polygon2], `bug_${bug_id}_input.json`);

var union = PolyBool.union(polygon1, polygon2);
console.log(JSON.stringify(union));

converter.store_polygons([union], `bug_${bug_id}_output.json`);
