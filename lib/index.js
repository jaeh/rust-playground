const addon = require('../native/index.node');

if (process.env.NODE_ENV !== 'test') {
  const res = addon.sha3(256, "testing");

  console.log(res);
}

module.exports = addon;
