require("dotenv").config();
const logger = require("console");
const path = require("path");
const webpack = require("webpack");


const STATIC_PATH = "/dist/";
const mode =
  process.env.NODE_ENV === "production" ? "production" : "development";
const PWD = process.cwd();
const resolvePath = (relativePath) => path.resolve(PWD, relativePath);
const env = {
  process: JSON.stringify({}),
  STREAMING_URL: JSON.stringify(
    process.env.STREAMING_URL || "http://localhost:90"
  ),
};

console.info(env);

const config = {
  mode,
  entry: {
    block: "./web/Index.tsx",
  },
  plugins: [
    new webpack.DefinePlugin(env),
  ],
  output: {
    publicPath: STATIC_PATH,
    path: resolvePath(`.${STATIC_PATH}`),
    filename: "[name].js",
    chunkFilename: "[name].bundle.js",
    libraryTarget: "umd",
  },
  devtool: "source-map",
  optimization: {
    minimize: mode === "production",
  },
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"],
  },
  module: {
    rules: [
      {
        test: /\.ts(x?)$/,
        exclude: [/node_modules/, /third_party/, /server/],
        use: {
          loader: "ts-loader",
        },
      },
      { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
      {
        test: /\.(s?)css$/,
        use: ["style-loader", "css-loader"],
        //use: ["style-loader", "css-loader", "sass-loader"],
      },
      {
        test: /\.(png|jpe?g|gif|woff|woff2|eot|ttf|svg)$/i,
        use: [
          {
            loader: "file-loader",
          },
        ],
      },
    ],
  },
};

if (mode === "development") {
  logger.log("Looks like we are in development mode");

  config.devServer = {
    port: 3000,
    hot: true,
    static: {
      directory: path.join(__dirname, 'web'),
    },
  }
}

module.exports = config;
