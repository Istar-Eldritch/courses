var readline = require('readline');

process.stdin.setEncoding('utf8');
var rl = readline.createInterface({
  input: process.stdin,
  terminal: false
});

rl.on('line', readLine);

let line_count = 0;
const lines = {};

function binary_search(items, _item) {
  let item = + _item;
  let left = 0;
  let right = items.length;

  if (items.length === 0) {
    return -1;
  }

  while (true) {

    if (right === left) {
      return -1;
    } else {
      let i = Math.floor((right - left) / 2) + left;
      let check = + items[i];
      if (item === check) {
        return i;
      } else if (item < check) {
        if (i === left) {
          return -1;
        }
        right = i;
      } else {
        if (i === right) {
          return -1;
        }
        if (right - left === 1) {
          left = right;
        } else {
          left = i;
        }
      }
    }
  }
}

function readLine (line) {
  line_count++;
  let l = line.split(' ');
  l.shift();
  lines[line_count] = l;
  if (line_count == 2) {
    lines[2].forEach((item, i) => {
      if(i !== 0) {
        process.stdout.write(' ');
      }
      process.stdout.write('' + binary_search(lines[1], item));
      if (i === lines[2].length - 1) {
        process.stdout.write('\n');
      }
    });
      process.exit();
  }
}
