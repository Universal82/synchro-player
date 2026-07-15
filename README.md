# This is synchro-player!
Synchro-player is a program that I'm making to solve a problem I haven't seen many other programs solve: how do you watch something with someone else over the internet without streaming to them? Synchro-player is the answer! Synchro-player plays local files and syncs the playstate and timestamp. This means you and your friends, as long as you're playing the same file, will have full resolution and see the same things. I achieve this by making the client send their timestamp each time they update the playstate: pausing, playing and even seeking--including frame by frame! The clients then pause or play whilst discreetly skipping to the exact timestamp specified by the client sending the action. This helps correct any desync passively.

# Todo:
- [ ] Build headless server and network interfaces to actually *do* the syncing thing
- [ ] Hook the client messages into the networking to send messages to the server too
