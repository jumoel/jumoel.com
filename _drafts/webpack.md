---
title: "From zero to webpack, one error message at the time"
categories: react
---

If you've ever struggled getting to grips with webpack, now is a good time to get back up on the horse. A stable release of webpack 2 is out, and this guide will take you from zero to a minimal, functional webpack configuration. The end result will be a small, but functional React application. The configuration will be expanded one small item at a time and will be driven by error messages. By not starting out with a boilerplate, you'll be able to understand what each single part does, and thus be able to expand upon it yourself when new needs arise.

## Getting up and running

**Prerequisites**: I assume some familiarity with React: what it is, what it does, that components are the building blocks and that the JSX syntax is a way to render components. At a later stage, familiarity with Express is assumed. If all of this is foreign to you, look at the [React documentation](https://facebook.github.io/react/docs/hello-world.html), and when the need arises, the [Express documentation](http://expressjs.com/en/starter/hello-world.html). The target group I have in mind when writing this is someone who has used a a boilerplate or `create-react-app` (and wanted to peek behind the curtain), but became overwhelmed when trying to modify or understand the build setup.

With that out of the way, let's start a new project:

```sh
$ mkdir webpack-project && cd webpack-project && npm init -y
```

Once that is done, we will need webpack itself:

```sh
$ npm install --save-dev webpack
```

Webpack provides a binary in the `node_modules` folder that can be run:

```sh
$ ./node_modules/.bin/webpack
No configuration file found and no output filename configured via CLI option.
A configuration file could be named 'webpack.config.js' in the current directory.
...
```

Take a look at the error message: Webpack needs some configuration before it can know what to do. Create a file named `webpack.config.js` with this content:

```js
module.exports = {}
```

Once again, run webpack and observe error:

```sh
$ ./node_modules/.bin/webpack
...
Error: 'output.filename' is required, either in config file or as --output-filename
...
```

We need to specify where webpack should store its output. Make `webpack.config.js` look like:

```js
module.exports = {
	output: {
		filename: 'bundle.js',
	},
};
```

Again, run webpack and observe error:

```sh
$ ./node_modules/.bin/webpack
Configuration file found but no entry configured.
...
```

Webpack needs something to build. Tell webpack to start the build with `src/index.js` by making `webpack.config.js` look like this:

```js
const path = require('path');

module.exports = {
	entry: path.resolve('./src/index.js'),
	output: {
		filename: 'bundle.js',
	},
};
```

Now, when you run webpack, the error message will look something like this:

```
ERROR in Entry module not found: Error: Can't resolve '/<path>/webpack-project/src/index.js' in '/<path>/webpack-project'
```

Make an empty file in `src/index.js` and run webpack again. `bundle.js` will appear, which means that webpack is working. But putting output files in the same folder will become cluttered quickly. To fix this, set `output.path` to `path.resolve('./dist')` in `webpack.config.js`:

```js
const path = require('path');

module.exports = {
	entry: path.resolve('./src/index.js'),
	output: {
		path: path.resolve('./dist'),
		filename: 'bundle.js',
	},
};
```

When you run webpack, it will now put the files in the `dist` folder, which is much better.

Now, lets make webpack output some actual code. Modify `src/index.js`:

```js
console.log('it works')
```

See that Webpack is still building and that the output bundle works:

```sh
$ ./node_modules/.bin/webpack && node ./dist/bundle.js
it works
```

## Transforming code with Babel

Let's add some modern code to `src/index.js`:

```js
class A {
	hello() { console.log('it works'); }
}

(new A).hello();
```

If you run a recent version of Node.js, this will run perfectly fine. If you want this code to work on older platforms or in browsers, the code will have to be transformed. We can do this with [Babel](https://babeljs.io/):

```sh
$ npm install --save-dev babel-core babel-cli babel-preset-env
```

Babel can be configured with a file called `.babelrc`:

```json
{
  "presets": [
		[ "env", {
			"targets": { "node": 4 }
		} ]
  ]
}
```

By specifying a target of Node.js v4, we can ensure that Babel actually does stuff with our code, no matter the version of Node.js is actually used.

Run Babel:

```sh
$ ./node_modules/.bin/babel src/index.js -o dist/bundle.js
```

Verify that it Babel transformed the code. Pay special attention to the part beginning with `var A = ...`:

```js
$ cat ./dist/bundle.js
'use strict';

var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

var A = function () {
	function A() {
		_classCallCheck(this, A);
	}

	_createClass(A, [{
		key: 'hello',
		value: function hello() {
			console.log('it works');
		}
	}]);

	return A;
}();

new A().hello();
```

Run webpack and build the same code. This also works, but the output (stashed between the webpack specific code) is different:

```js
$ ./node_modules/.bin/webpack && cat dist/bundle.js
/* start of file omitted for brevity */
class A {
        hello() { console.log('it works'); }
}

(new A).hello();
/* end of file omitted for brevity */
```

Let's add someting to make it output the same as the `babel` process. When you add the following section to `webpack.config.js`, you are telling webpack how to process [modules](https://webpack.js.org/concepts/modules/) that pass the filename test. In this case webpack will process all javascript files with Babel:

```js
// Insert this at the top level of the export from webpack.config.js
module: {
	rules: [
		{
			test: /\.js$/,
			include: path.resolve('./src'),
			loader: 'babel-loader',
		}
	],
},
```

If you build with webpack and take a look at the output in `dist/bundle.js`, you'll see that it now doesn't contain the native class code anymore (which means it works in Node 4 as we requested).

We've already come a long way, but before we get something that is more useful, lets clean up a bit.

Instead of calling webpack directly, add a `build` script in `package.json` that calls webpack. That way `npm run build` can be used instead of `./node_modules/.bin/webpack`:

```json
...
{
	"scripts": {
		"build": "webpack"
	}
}
...
```

I also suggest running `npm uninstall babel-cli`, because it won't be used any longer.

We'll also change `.babelrc` to `babelrc.js` and modify it like so, since it's no longer required to be JSON:

```js
module.exports = {
  presets: [
		[ 'env', {
			targets: { node: 'current' }
		} ]
  ]
}
```

Babel only natively knows about `.babelrc` files, so to pick up the new file, the `babel-loader`-rule in `webpack.config.js` needs updating:

```js
{
	test: /\.js$/,
	include: path.resolve('./src'),
	loader: 'babel-loader',
	query: require('./babelrc.js'), // Add this line
}
```

This will all make it easier to extract, reuse and extend the build configuration at a later stage.

## Rendering with React

Now that the javascript is being processed correctly by both webpack and babel, let's add React.

```sh
$ npm install --save react
```

Put some React code in `src/index.js`:

```js
import React from 'react';

class HelloWorld extends React.Component {
	render() {
		return <h1>Hello, World!</h1>;
	}
}

console.log(new HelloWorld().render());
```

Try to build it with webpack:

```sh
$ npm run build
```

Notice that it fails on the `<h1>` from our component. To actually convert the JSX tag in our React code to something that Node.js/browsers understand, an additional preset for babel is needed:

```sh
$ npm install --save-dev babel-preset-react
```

Add it to the presets attribute of the babel configuration:

```js
module.exports = {
  presets: [
		[ 'env', {
			targets: { node: 'current' }
		} ],

		'react',
  ],
}
```

Build the project, and verify that the resulting code actually works:

```js
$ npm run build && node ./dist/bundle.js
{ '$$typeof': Symbol(react.element),
  type: 'h1',
  key: null,
  ref: null,
  props: { children: 'Hello, World!' },
  _owner: null,
  _store: {} }
```

Because we would like to use React to build websites, let's get the component rendered in a browser instead of a terminal.

First, we need some basic HTML to bootstrap the process. Put the following in `src/index.html`.

```html
<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>React App</title>
</head>
<body>
	<script src="../dist/bundle.js"></script>
</body>
</html>
```

If you open this in your browser and open the Developer Console, you should see the same output as you saw in your terminal, just represented in a different way.

To get the component to render, refactor a bit:

 * Change `src/index.js` to `src/HelloWorld.js`.
 * Remove the `console.log`.
 * Export the `class`, so it can be used from another file.

The contents of `HelloWorld.js` should look like this:

```js
import React from 'react';

export default class HelloWorld extends React.Component {
	render() {
		return <h1>Hello, World!</h1>;
	}
}
```

To have React render into the DOM, we will need the `react-dom` package:

```sh
$ npm install --save react-dom
```

We'll also need a new file to serve as the entrypoint for the browser. The reason for creating a seperate entrypoint is to make it easier to make the application universal in the future.

ReactDOM needs somewhere to render its results to. Add the following to the `<body>` in `src/index.html`:

```html
<div id='root'></div>
```

Now, add the browser entrypoint in `src/index.browser.js` to get React to render and control the DOM under `#root`:

```js
import React from 'react';
import ReactDOM from 'react-dom';
import HelloWorld from './HelloWorld';

const root = document.getElementById('root');

ReactDOM.render(<HelloWorld />, root);
```

Try to compile (with `npm run build`) and notice that it doesn't work anymore.

To get webpack to build our bundle again, change the entrypoint in `webpack.config.js` to `./src/index.browser.js`. At the same time, change the `target` in `babelrc.js` to something actually representing browsers instead of Node.js:

```js
module.exports = {
  presets: [
		[ 'env', {
			targets: { browsers: '> 5%, last 2 versions' }
		} ],

		'react',
  ],
}
```

Build the project, refresh `index.html` in the browser and you should see a pretty `<h1>Hello World</h1>` rendered in all its glory.

If you have the React Devtools installed (if not visit https://fb.me/react-devtools), you'll see the following notice:

> Download the React DevTools and use an HTTP server (instead of a file: URL) for a better development experience

Let's fix that part.

## Adding a server

```sh
$ npm install --save express
```

Add `src/index.server.js` with the following:

```js
const path = require('path');
const express = require('express');

const app = express();

app.get('*', (req, res) => {
	res.sendFile(path.resolve(__dirname, './index.html'));
});

app.listen(3000, () => {
	console.log('React app listening on port 3000!')
})
```

Run the server with `node ./src/index.server.js` open [localhost:3000](http://localhost:3000) and you'll notice that it doesn't contain the "Hello World" text. If you open the Developer Tools, you can see that the `bundle.js` file isn't transferred correctly. This is caused by every request being served by the `get('*', ...)`, which just sends the contents of `index.html`. To fix this, add the following line to `index.server.js`:

```js
app.use('/static', express.static(path.resolve(__dirname, '../dist')));
```

You will also need to change the `src` attribute of the `<script>` tag in `index.html`:

```html
<script src="/static/bundle.js"></script>
```

Restart the server, reload your browser and behold "Hello World" in all its glory again.

# TODO

	* Convert server requires to imports
	* Add server webpack config part
	* Render react component on server
	* Add the client code to the server response
	* Add environments and different configs
	* Put the build pipeline in its own package
