#!/usr/bin/env node

// Test if polybooljs can handle interior/exterior properly.

var PolyBool = require('polybooljs');

let polygons = [
  {"regions":[[[0,0],[100,0],[100,10],[0,10]]],"inverted":false},
  {"regions":[[[90,0],[100,0],[100,100],[90,100]]],"inverted":false},
  {"regions":[[[0,90],[100,90],[100,100],[0,100]]],"inverted":false},
  {"regions":[[[0,0],[10,0],[10,100],[0,100]]],"inverted":false},
]

/*
var union = polygons[0];
for (var i = 1; i < polygons.length; i++) {
  union = PolyBool.union(union, polygons[i]);
  console.log(JSON.stringify(union));
}
*/

var segments = PolyBool.segments(polygons[0]);
for (var i = 1; i < polygons.length; i++){
  var seg2 = PolyBool.segments(polygons[i]);
  var comb = PolyBool.combine(segments, seg2);
  segments = PolyBool.selectUnion(comb);
}
var union = PolyBool.polygon(segments);

console.log(JSON.stringify(union));


