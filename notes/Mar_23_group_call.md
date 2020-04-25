# March 23, video call meeting # 1

### Note From Original 'Coalesce' app team
Viktor, Guillem


#### Viktor 
- envisioned the app as a meetup.com alternative that could double as an event center mgmt booking, inventory
- excited about the idea of self organizing events (the goal of the newer team members from Portland Hackathon)
- onboard for this starting point

Proposal: see where these concepts differ and how it could extend to a meetup replacement


#### Guillem 
- 2 apps interoperable

Offer: I can support from technical side

#### peter

Proposal: 'Gather' as the name for the app (rather than 'Coalesce' or 'Converge')

brings up issue with running holochain package build (looks for any .json file rather than the app specific app.json)

- [ ] TODO update?

temp solution: have a bash script that moves it when you package and puts it back when done

#### Guillem

proposal: use case of app for organizing virtual meetups eg for software teams coordinating

#### Peter 

response: scope of project is general enough to include that use case

#### Feli 

- currently one person creates a general event and invites people, 

proposal: scope expands so floating 'criteria' can create events


#### Kamal 

- 'possibility' is the terminology we've been using for an event proposal

#### Peter

proposal: we check out user interface designs (from Kamal)


#### Kamal

shares screen, explains dashboard, 'possibility', 'criteria', 'roles'

#### Peter 

 we have 'create possibility' function tested

- [ ] TODO we need to add functionality for response, and measure metrics for response


#### Kamal 

proposal: we discuss what 'roles' are

#### Maddie 

- 'roles' and 'people' can be the same

#### Guillem 

- example of role of room provider

#### Peter 

- confusion of overlap of 'roles' and 'criteria'

#### Feli 

proposal: 'criteria' can be noted by event organizer to generate a 'role'


#### peter

- right now we're only tracking 'criteria' in the 'possibility' object


- [ ] TODO address confusion of role, person, criteria

 - how do these translate to the data stuctures of 'responses'?


#### Maddie 

proposal: we think up possible use-case scenarios to think through what we have so far

#### Peter 

proposal: we start user-testing the interface to see how people interact with the app

proposal: viktor can help with how he would use it and what would be addressed to meet the needs

#### Viktor 

- what got him excited was the need of self organizing events, common in burner community (burning man)

- resources can be fulfilled by people who arent in the event

#### Peter 

- use cases we've noted so far are 'software project' 'burner event' 'game night' think thru what the relevent terms are for each

#### Kamal 

-  there's a similar app 'comakery'

#### Guillem 

offer: help to make sure the app is interoperable 'mutual-credit' (the design can interface with 'currency' hApps)

#### Kamal 

proposal: reaching out to comakery founder, pick his brain see if he's open to collab

invitation: open invite to join meeting

#### Guillem 

design proposal: including a query by location feature (a functionality he is going to work with and make a library for holochain generally)

offer:share in future weeks

- [ ] TODO update?


#### Stephen -

- [ ] TODO translate designs to functional code, building front end using JAM stack, needs to be tied in to backend after,

- suggests a possible test engine

proposal: meeting with kamal about front end

- [ ] TODO update?

Meeting Adjourned (aprox 1 hr)
