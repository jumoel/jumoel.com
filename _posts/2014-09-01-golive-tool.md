---
title: "Golive: A simple deployment tool"
categories: infrastructure
date: 2014-09-01 13:49:00
---

I've heard that the best way to learn a new programming language is to create a
proper project in it. `golive` is my "Learn Golang"-project. It is a simple
deployment tool that listens to webhook requests from git repository providers
such as Bitbucket and Github. If the pushed repository and branch matches
something in the configuration file, corresponding jobs run.

`golive` is based on a simple JSON-based configuration file:

{% highlight json %}
{% raw %}
{
  "https://bitbucket.org/jumoel/test/": {
    "master": [
      "echo 'Commit from: {{.Repository}}{{.Branch}}' > test.txt"
    ]
  }
}
{% endraw %}
{% endhighlight %}

If I've set up POST hooks in my Bitbucket repository to point to a server where
`golive` is running, and I push some changes in the `master` branch, the `echo ...`
action is run. If multiple actions for a single branch are required, they are
added as strings in the array. If multiple branches are required, additional
keys are added:

{% highlight json %}
{
  "https://bitbucket.org/jumoel/test/": {
    "master": [...],
    "test": [...]
  }
}
{% endhighlight %}

The same goes for repositories:

{% highlight json %}
{
  "https://bitbucket.org/jumoel/test/": { ... },
  "https://some.other/git/repo/": { ... },
}
{% endhighlight %}

When the configuration file is changed by being overwritten or having changes
written to it, it is automatically reloaded.

At my current job we use it together with [Ansible](http://www.ansible.com)
playbooks to deploy websites across a number of servers. `golive` listens for
changes in our server provisioning repo and can thus update its own
configuration.

At the moment, only Bitbucket is supported, but Github support [is
underway][ghissue].

You can find the code at
[github.com/jumoel/golive](https://www.github.com/jumoel/golive). If you want to
give it a try, the binary is nothing more than a `go get
github.com/jumoel/golive` away.

As mentioned, this is my first proper project in Go, so any and all feedback is
very welcome.

  [ghissue]: https://github.com/jumoel/golive/issues/5
