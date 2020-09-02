---
title: Using Tailwind classnames in React Native with intellisense
categories: infrastructure
date: 2020-09-02 10:00
---

I'm writing a small app using [React Native](https://reactnative.dev). Since I'm a fan of [Tailwind](https://tailwindcss.com) for styling on the web, I wanted to use it for the app as well. The package [`tailwind-rn`](https://www.npmjs.com/package/tailwind-rn) is available, but requires the use of `tailwind('...')` to apply styles, which [doesn't work](https://github.com/tailwindlabs/tailwindcss-intellisense/issues/129) with the [intellisense plugin for VSCode](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

Wouldn't it be nice to both be able to use regular `className` props _and_ get intellisense working, all at the same time? Of course it would.

Using a small higher-order component (HOC), it's doable:

```jsx
import * as React from "react";
import tailwind from "tailwind-rn";

function getDisplayName(WrappedComponent) {
  return WrappedComponent.displayName || WrappedComponent.name || "Component";
}

export function withTailwind(Component) {
  function ComponentWithTailwind({ className, style, ...rest }) {
    const classes = className
      ? Array.isArray(className)
        ? className.flat().filter(Boolean).join(" ")
        : className
      : "";

    return <Component style={[tailwind(classes), style && style]} {...rest} />;
  }

  ComponentWithTailwind.displayName = `withTailWind(${getDisplayName(
    Component
  )})`;

  return ComponentWithTailwind;
}
```

By re-exporting the relevant components from `react-native`, `className` props are then available everywhere:

```js
import * as RN from "react-native";
import { withTailwind } from "./withTailwind";

export const Text = withTailwind(RN.Text);
export const View = withTailwind(RN.View);
// ... etc
```

The HOC allows both for string and array props. Autocomplete works for both. 🎉

```jsx
import { View, Text } from "./Base";

function Test() {
  return (
    <View className="bg-white">
      <Text className={["text-black"]}>Success!</Text>
    </View>
  );
}
```

![Working Tailwind intellisense in VSCode](/images/tailwind-intellisense.png)
