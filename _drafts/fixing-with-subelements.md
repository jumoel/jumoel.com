---
title: "Fixing `with_subelements` in Ansible"
categories: infrastructure
date: 2014-09-17 08:03:00
---

Lately I've been working on automatically provisioning servers [while trying to
remain sane][ansidem] as well as getting [simple automatic deployments][golive]
running.

I've been bit by it before, but it wasn't until I was trying to set up
loadbalancers and DNS in Rackspace, that I took some time to look into the
problem: [`with_subelements`][withsub] doesn't work properly. At least not the
way I *thought* it should work.

With this object...

{% highlight json %}
{
  "object": {
    "subobject": [
      {
        "sublist": [ … ]
      },
      {
        "sublist": [ … ]
      },
      ⋮
    ]
  }
}
{% endhighlight %}

... `with_subelements` can access the list in `object.subobject` as well as the
list in `sublist` with the following:

{% highlight yaml %}
  - name: Something
    some_module: …
    with_subelements:
      - object.subobject
      - sublist
{% endhighlight %}

But if `sublist` is placer deeper in the object, like so:

{% highlight json %}
{
  "object": {
    "subobject": [
      {
        "xyz": {
          "sublist": [ … ]
        }
      },
      {
        "xyz": {
          "sublist": [ … ]
        }
      },
      ⋮
    ]
  }
}
{% endhighlight %}

... and you try to access it with

{% highlight yaml %}
    …
    with_subelements:
      - object.subobject
      - xyz.sublist
{% endhighlight %}

... Ansible just complains that the key `xyz.sublist` wasn't found or didn't
contain a list.

I [fixed that][myfix], which you can place in your Ansible project in the
`filter_plugins` folder until my [pull request][pr] upstream might accepted.

  [ansidem]: /2014/ansible-rackspace-idempotence.html
  [golive]: /2014/golive-tool.html
  [withsub]: http://docs.ansible.com/playbooks_loops.html#looping-over-subelements
  [myfix]: https://github.com/jumoel/ansible/blob/dc918f78dc0a7e7f23f451df2e1365e7ff172312/lib/ansible/runner/lookup_plugins/subelements.py
  [pr]: https://github.com/ansible/ansible/pull/9005
