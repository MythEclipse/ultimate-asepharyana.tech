module.exports = {
  apps : [{
    name   : "RustExpress",
    script : "./target/release/RustExpress",
    instances: "max",
    exec_mode: "cluster",
    env: {
      "NODE_ENV": "production"
    }
  }]
};