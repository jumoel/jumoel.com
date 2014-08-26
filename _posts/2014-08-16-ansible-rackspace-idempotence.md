---
title: Ansible, Rackspace and Idempotence
categories: infrastructure
date: 2014-08-26 22:35:00
---

I was recently tasked with transitioning a server setup from the medieval times
of "we can't get in contact with the server-guy, but here are some vague
instructions that are probably outdated" to the modern times of "automate all
the things!" Trying to make sure the process was fully codified and idempotent,
I ran into some snags here and there. This is a documentation of those snags,
both for a future me, but also for others, should they struggle with the same
things. As the title says, it is focused on [Ansible][ans] and its integration
with [Rackspace][rax].

By the way - I am by no means an Ansible expert, so if any of what I write is
pure bollocks and due to my own incompetence, please [get in touch][tw]. Now,
with that out of the way...

I chose [Ansible][ans] setting up the servers we needed. Initially, I would have
liked to try out [Terraform][tf] to administer the infrastructure itself, but
because it wasn't available for [OpenStack][tfos], Ansible would also have to
suffice for this.

Ansible has a whole slew of [cloud
modules](http://docs.ansible.com/list_of_cloud_modules.html), so ensuring the
initial availability of servers and embedded SSH keypairs was easy and just
required the basic use of the [rax][raxmod]- and [rax_keypair][raxkey]-modules,
a `with_items` and a `cloudservers` variable in a YAML file.

The problems arose when I wanted to bootstrap the servers so SSH wasn't running
on port 22 and root login (the default Ansible user) wasn't allowed. To allow
Ansible to run effortlessly through the new user, passwordless `sudo` was also
needed. After this, I wanted the playbook to be able to continue to the regular
setup based on server roles and I didn't want to split the playbook up into
multiple files, because that would heighten the requirement for external
documentation. My goal was that a simple `ansible-playbook serversetup.yml` was
all that was needed.[^autoupdate]

Because the SSH configuration changes half way through the provisioning, I was
looking for some way to mark whether or not a specific server. Lo an behold the
`meta` property on the `rax` module, in which I added `bootstrapped: False`.
This turned out to be my first problem. When I later in the playbook changed it
to `bootstrapped: True` and then tried to run the playbook again, a duplicate
set of servers were created. The Rackspace module apparently looked to see if
there were any servers that matched all of the specified parameters, including
the metadata. Since the existing servers had `bootstrapped: True` and not
`False`, a new, correct set (from the eyes of Ansible/Rackspace) was created.

I found out that the Rackspace inventory script creates groups based on
metadata, so my server provisioning part was enriched with the following
condition, which solved the duplication:

{% highlight yaml %}
{% raw %}
  when: (groups.meta_bootstrapped_True is not defined) or ({{item.hostname}} not in groups.meta_bootstrapped_True)
{% endraw %}
{% endhighlight %}

I haven't investigated if the same problems are present with the `group` and
`exact_count` parameters, because I didn't want to be forced to have servers
named `1.somesystem.example.com`, instead of just `somesystem.example.com`.

After any new servers have been spun up (Rackspace can be very slow, by the way)
the playbook makes sure to gather any servers that have been created at some
earlier playbook run, but for some reason haven't been bootstrapped
yet.[^failures] This is done with the [rax_facts][raxfac]-module and a
modification of the above `when`-condition. Instead of `item.hostname`,
`inventory_hostname` is used.

The bootstrap script then runs on any unbootstrapped servers and makes sure root
login is disallowd, the SSH port is changed and that a special Ansible user is
added with a global SSH key attached, after which all is fine.The only minor
gotcha at this point is that it's not a good idea to change the Ansible SSH
variables before the SSH daemon has been restarted because Ansible then tries to
log in and restart SSH on the new port which isn't active yet. Lesson learned.

The next problem arose when I tried to be smart and add the bootstrapped SSH
connection info to a group variables file. I specify the required SSH port and
remote user (22 and *root*) in the bootstrapping playbook, so I thought I could
just put the default, bootstrapped values in `group_vars/LON.yml`. A group is
automatically created with our Rackspace servers in the London region, but
unfortunately newly provisioned servers apparently aren't added to this group. I
tried manually adding them, but for some reason that didn't go through either.
In the end, I put the SSH variables in `group_vars/all.yml`, even though I don't
really like polluting the 'global' namespace in that way. There has to be a
prettier way to handle this, but that's for another time.

I encountered a few other hiccups along the way, but they were mostly caused by
our half-weird server requirements and aren't as generally applicable as (I
think) the above was. Ansible is great and (oftentimes) simple, but I can still
long for a proper programming language to define server setup in[^notthat]. If
Ansible corresponds to [Grunt][grunt], what corresponds to [Gulp][gulp] in this
relationship?

But now I am wiser, the servers are running, and I am able to `Ctrl-C` all I
want. The best part is, that if I should be unavailable and the servers are on
fire, `ansible-playbook serversetup.yml` is all that is required *and*
modifications can be done by people other than greybeard unix
hackers[^greybeard], which was the goal all along.

  [tf]: http://www.terraform.io
  [tfos]: https://github.com/hashicorp/terraform/issues/51
  [ans]: http://www.ansible.com/
  [rax]: http://www.rackspace.com/
  [raxmod]: http://docs.ansible.com/rax_module.html
  [raxkey]: http://docs.ansible.com/rax_keypair_module.html
  [raxfac]: http://docs.ansible.com/rax_facts_module.html
  [grunt]: http://gruntjs.com
  [gulp]: http://gulpjs.com
  [tw]: https://twitter.com/intent/tweet?screen_name=jumoel&text=Here's%20what%20you%20SHOULD%20have%20done...

  [^autoupdate]: Part of my goal was to allow the servers to autoupdate
    themselves via repository webhooks, but I haven't got the higher-ups to
    agree on that yet -- too risky, they think. But I want it to be easy to do
    at a later stage, should I convince them.

  [^failures]: The best way I could think of being able to handle network and/or
    hardware failures, was to make the playbook robust enough, that it could be
    interrupted with `Ctrl-C` at all times. So that's what I did. `Ctrl-C`'ed at
    all times.

  [^notthat]: No, Puppet and Chef does not count.

  [^greybeard]: Not that there is anything wrong with greybeards; they can just
    be hard to find nowadays.
