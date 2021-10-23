#!/usr/bin/env node
// Test martinez bug https://github.com/w8r/martinez/issues/110 with Polybooljs

// https://medium.com/@nsisodiya/the-ultimate-vs-code-debug-setup-for-node-js-a03cdbc594ba
// node -r esm --inspect-brk --inspect=0.0.0.0:9229 ./debug_martinez_110_w8r.js

// We need to run this with:
// node -r esm ./debug_martinez_110_check.js
// To make node work with ES6 modules...
// https://stackoverflow.com/questions/45854169/how-can-i-use-an-es6-import-in-node

const converter = require("./converter_w8r");
const exec = require('child_process').exec;

var bug_id = 110;

//const martinez = require("../../geo-clip/references/martinez/src/index");
//import * as martinez from "../../geo-clip/references/martinez/src/index";
//import * as martinez from "../../geo-clip/references/martinez/index";
import * as martinez from "../../../geo/martinez/index";

//const p1 = [[[0, 0], [100, 200], [200, 300], [0, 0]]];
//const p2 = [[[20, 20], [60, 300], [100, 120], [20, 20]]];

// Variations changing first:
// - x 115 => 116
// - x 115 => 114
// - y 96 => 97
// Variants changing third (the point that cuts the short segment):
// - x 120 => 121 fixes problem
// - x 120 => 119 creates problem with tiny hole
//const p1 = [[[115,96], [140,206], [120,210], [125,250], [80,300], [115,96]]];
//const p2 = [[[111,228], [129,192], [309,282], [111,228]]];

const p1 = [[
  [180.60987101280907, 22.943242898435663],
  [280.6098710128091, 22.943242898435663],
  [280.6098710128091, 62.94324289843566],
  [180.60987101280907, 62.94324289843566],
  [180.60987101280907, 22.943242898435663]
]];
const p2 = [[
  [-5.65625, 110.828125],
  [-7.53125, 202.234375],
  [366.0625, 202.234375],
  [356.6875, 65.828125],
  [260.125, 59.265625],
  [253.09375, 40.984375],
  [189.34375, 19.890625],
  [141.0625, 36.765625],
  [111.53125, 6.765625],
  [73.5625, 36.765625],
  [67.46875, 10.984375],
  [41.21875, 10.515625],
  [36.0625, 42.390625],
  [65.59375, 53.171875],
  [-5.65625, 110.828125]
]];

let union = martinez.union(p1, p2);

console.log("raw output:", JSON.stringify(union));

const fn_i = `bug_${bug_id}_w8r_input.json`;
const fn_o = `bug_${bug_id}_w8r_output.json`;
converter.store_polygons([p1, p2], fn_i);
converter.store_polygons(union, fn_o);

// [[[[0,0],[24,48],[87.6923076923077,175.3846153846154],[100,120],[20,20]]],[[[100,200],[200,300]]]]
// [[[[0,0],[24,48],[87.6923076923077,175.3846153846154],[100,120],[20,20]]],[[[100,200],[200,300]]]]

exec(`../../scripts/plot_world.py ${fn_i} ${fn_o}`, function callback(error, stdout, stderr){
  // result
});

/*
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
*/