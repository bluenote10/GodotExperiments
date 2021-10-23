#!/usr/bin/env node
// Test martinez bug https://github.com/w8r/martinez/issues/110 with Polybooljs

const converter = require("./converter");

var bug_id = 102;

var PolyBool = require('polybooljs');

let polygon1 = {
  "regions":[
    [[34, 119], [20, 20], [132, 141], [34, 119]]
  ],"inverted":false
}
let polygon2 = {
  "regions":[
    [[27.681744389656238, 74.32090675542626], [25.1362898442017, 56.3209067554263], [35.67034343592924, 56.32090675542627], [35.67034343592924, 74.32090675542624], [27.681744389656238, 74.32090675542626]]
  ],"inverted":false
}

converter.store_polygons([polygon1, polygon2], `bug_${bug_id}_input.json`);

var union = PolyBool.intersect(polygon1, polygon2);
console.log(JSON.stringify(union));

converter.store_polygons([union], `bug_${bug_id}_output.json`);
