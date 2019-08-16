Small notes on how to test the new context API in React with Enzyme.

**Before:**

```jsx
shallow(<Component />).instance().getChildContext()
```

*or*

```jsx
shallow(<Component><Child /></Component>).find(Child).context()
```

**Now:**

```jsx
shallow(
  <Component>
    <Context.Consumer>
      { context => <ContextGrabber context={context} /> }
    </Context.Consumer>
  </Consumer>
);
```
