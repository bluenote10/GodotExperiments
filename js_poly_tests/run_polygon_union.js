#!/usr/bin/env node

var PolyBool = require('polybooljs');
const fs = require('fs');


function convert_input(data) {
  let polygons = data.map(polygon => {
    let exterior = polygon["exterior"]
    let interiors = polygon["interiors"]
    let points = convert_input_points(exterior);
    return {
      regions: [points],
      inverted: false,
    }
  })
  return polygons
}

function convert_input_points(points) {
  points.pop()
  return points.map(p => [p["x"], p["y"]])
}

function convert_output(polygon) {
  return (
    polygon["regions"].map(region => {
      region.push(region[0])
      return {
        exterior: region.map(point => { return {x: point[0], y: point[1]}}),
        interiors: [],
      }
    })
  );
}

var args = process.argv.slice(2)

if (args.length != 2) {
  console.log(process.argv);
  console.log("ERROR: Wrong number of arguments.")
} else {
  var filename_in = args[0];
  var filename_out = args[1];

  let data = JSON.parse(fs.readFileSync(filename_in));
  console.log(data);

  let polygons = convert_input(data);
  console.log(JSON.stringify(polygons));

  var union = polygons[0];
  for (var i = 1; i < polygons.length; i++) {
    union = PolyBool.union(union, polygons[i]);
    console.log(JSON.stringify(union));
  }

  /*
  var segments = PolyBool.segments(polygons[0]);
  for (var i = 1; i < polygons.length; i++){
    var seg2 = PolyBool.segments(polygons[i]);
    var comb = PolyBool.combine(segments, seg2);
    segments = PolyBool.selectUnion(comb);
  }
  var union = PolyBool.polygon(segments);
  */

  console.log(JSON.stringify(union));

  let output_converted = convert_output(union);
  console.log(JSON.stringify(output_converted));
  fs.writeFileSync(filename_out, JSON.stringify(output_converted));
}

