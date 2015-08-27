---
title: "Fixing `with_subelements` in Ansible"
categories: infrastructure
date: 2014-09-17 08:03:00
---

Lately I've been working on automatically provisioning servers [while trying to
remain sane][ansidem] as well as getting [simple automatic deployments][golive]
running. I've been bit by it before, but it wasn't until I was trying to set up
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
- …
  with_subelements:
    - object.subobject
    - sublist
{% endhighlight %}

But if `sublist` is placed deeper in the object, like the virtual IPs are, when
creating multiple Rackspace load balancers:

{% highlight json %}
"lb.results": [
    {
        "balancer": {
            …,
            "virtual_ips": [
                {
                    …,
                    "ip_version": "IPV4",
                    "type": "PUBLIC"
                },
                {
                    …,
                    "ip_version": "IPV6",
                    "type": "PUBLIC"
                }
            ]
        },
        …
    },
    …
]
{% endhighlight %}

... and you try to access it with

{% highlight yaml %}
- …
  with_subelements:
    - lb.results
    - balancer.virtual_ips
{% endhighlight %}

... Ansible just complains that the key `balancer.virtual_ips` wasn't found or
didn't contain a list.

So I [fixed that][myfix], making `with_subelements` properly descend  the levels
of an object. You can place the updated version of `subelements.py` in your
Ansible project in the `filter_plugins` folder until my [pull request][pr]
hopefully is accepted.

  [ansidem]: /2014/ansible-rackspace-idempotence.html
  [golive]: /2014/golive-tool.html
  [withsub]: http://docs.ansible.com/playbooks_loops.html#looping-over-subelements
  [myfix]: https://github.com/jumoel/ansible/blob/dc918f78dc0a7e7f23f451df2e1365e7ff172312/lib/ansible/runner/lookup_plugins/subelements.py
  [pr]: https://github.com/ansible/ansible/pull/9005
