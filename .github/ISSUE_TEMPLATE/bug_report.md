---
name: Bug report
about: Create a report to help us improve
title: ''
labels: bug
assignees: flukejones

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behaviour:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Please provide the following:**
 - Distro:
 - Kernel (`uname -r`):
 - `cat /sys/class/dmi/id/product_name`:
 - `cat /sys/class/dmi/id/product_family`:
 - `cat /sys/class/dmi/id/board_name`:
 - `lsusb |grep 0b05`
   + `sudo lsusb -vd 0b05:<all devices from above> > ~/laptop_info`, and attach to issue.
 - `sudo journalctl -b --unit rog-core > rog-core.log` and attach to issue.
 - `sudo usbhid-dump > usbhid.log` and attach to issue.

**Additional context**
Add any other context about the problem here.
