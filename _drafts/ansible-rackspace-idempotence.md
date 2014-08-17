---
title: Ansible, Rackspace and Idempotence
categories: Infrastructure
---

Recently, I was tasked with setting up some server infrastructure on Rackspace.
Since [Terraform][tf] isn't yet available for [OpenStack][tfos], I opted to use
[Ansible][ans] for both infrastructure setup as well as provisioning. When doing
so, I encountered some difficulties ensuring that the setup was idempotent.

There were some problems with the previous server setup: Only a single person
knew how to set them up --- manually. And no one was quite sure what would happen
if the software on them was upgraded, so it wasn't.

To bring the servers into the modern age I wanted to keep as much as possible --
both infrastructure and software setup -- automated and checked into version
control. This was to ensure consistency between the different servers as well as
advocate a degree of ephemerality. The latter would discourage manual changes on
the servers, because they might be replaced often. It should also be easy to
spin up new servers to test new environments as well as software upgrades.

 - first steps, easy
 - try to add new user, ssh key, change ssh port, problems
 - idempotence and metadata - metadata part of rackspace checking if server present
 - custom groups and set logic
 - problems now:
   - critical point between ssh port change and metadata setting
   - under-creation instance says that it is available in pyrax - wait until
     progress is 100?

  [tf]: http://www.terraform.io
  [tfos]: https://github.com/hashicorp/terraform/issues/51
  [ans]: http://www.ansible.com/
