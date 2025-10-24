-LilButter---ButterClient


To connect to the server make sure you installed the patch under the SERVERS folder
I included the Erupe 9.3 patch files (drag and drop) then compile
Erupe 9.2 server in whole with the patch pre-done (uses signv2 instead of api) <-- files patched are the same as 9.3 just signv2 instead of api
Under extra is the Tent feature <- not required by anymeans nor did I make it. I was playing around with the feature and thats just the files I adjusted for it I think I translated some of it I dont remember.


________________________________________________________________________________________________________________________________________

ButterClient: Things that still need to be done/added

Frontend:

- Clean up endpoints and remove depricated functions
- Double check patchserver is read only for security measures (you can also just change the folder to read only)
- Add linux support <- huge rework of frontend to handle login for linux (or lazy route and just fix the cli lol)
- UI cleanup/ css cleanup
- Aditional Character info to display in unit cards (optional)
- Adjust port for Unit cards

Backend:

- Adjust friends list handling (in mhf.rs under mhfiel) <--currently only on windows 10 and 11
^^ currently just injects after the game launches but I'd like to figure out a way to store then load that info without injection

________________________________________________________________________________________________________________________________________

Folder/File Locations:

Public = All images, backgrounds, unit cards ect
dist = All images, backgrounds, unit cards ect
^^although im pretty sure it just pulls from the public folder but for now just add the image to both to be safe.

locale = both localization files for eng and jap.
^^^^ japenese file needs these added:
reset-patch-label = Maintenance
reset-button-label = Reset patched files
resetting-label = Resetting…

package.json and package-lock.json include the frontend window title <-- if you wish to change it.

index.html = intial window

src = All the frontend + background images (I know kinda scuffed lol)
    -Classic = Classic view/ main view of the frontend
	-modern = Modern view of frontend (THEY ARE SEPERATE SO MAKE SURE YOU CHANGE/ADJUST BOTH)
	-settings = Settings page for both the modern and classic view besides the left tabs
	-main.js = the initial window before it loads classic or modern views but after index.html
	-store.js = All the saved/adjustable settings + offlinemode (IMPORTANT)
	-style.css = most of the basic styling although not all styling is in it, which is why I put css cleanup since it needs to be moved into it.
	-sfx.js = the sound effects handling
	-manifest.rs = stores the current server ip for the patchserver

Note: messagelist.json and serverlist.json in the main dir are depricated. you can find that info in the store file
	
src-tauri = All the backend/ the actual game
    - src = all the backend data and endpoints for the frontend to use
	    -config.rs = the starting default of the front end via classic or modern view
		-endpoints.rs = what communicates to the server (needs some cleanup)
		-main.rs = data that gets passed into mhf-iel + some others
		-manifest.rs = New file that checks the manifest and byte size of the update files + checks selected server to see if it needs to update or not.
		-patcher.rs = New file the handle server patches for the client (not complete) <-- needs better handling for .butterold files / cleanup
		-server.rs = Server data sent to the backend/main.rs
		-settings.rs = Game ini settings like resolution and ect <-- needs to be tweaked there is a bug that doesnt change the resolution properly
		-store.rs = store helper
		-user.rs = App name and login
		
	mhf-iel-master = Main backend for actually loading the game after login.
        -src = Everything in this folder is what is used for the backend
	        -error.rs = basic error logs
		    -lib.rs = data for main.rs and for basic connection data example: mezfest and mhfconfig
		    -mhf.rs = THE MAIN FILE handles all offests/pointers for all the required data that gets displayed in the game. <-- if something is missing in game this is the file to look at.
		    ^^^note on this one I use x32debug to find the pointers/offsets. EXTRA NOTE the offests/pointers will be different from the normal launcher so debug the ButterClient to actually find the correct spot.
		    -utils.rs = gloabl allocate and mutex name
		
    mhf-iel-cli = bare bones login/launch for mhf-iel (I have not updated this but could be tweaked to get it working on linux)
    ^^^also is not used in the butterclient but I kept the files there anyway
	
	icons = The game's/window favicon after launching

________________________________________________________________________________________________________________________________________

HOW TO BUILD:
First run: rustup override set nightly

# dev mode
npm run tauri:dev

# build for release
npm run tauri:build


After compiling it will be under src-tauri/target/i686-pc-windows-msvc

Happy hunting!!



________________________________________________________________________________________________________________________________________

EXTRA EXTRA (lol I know, I'm sorry)

New ports that are need for this are 8094 and 8090
8094 is the port for the patchserver which can be found under the PATCHSERVER folder in the SERVER FILES
8090 is the port for the image server which hosts the backgrounds and announcements from the server side <-- these can be live updated
^^note these ports can be changed however for the image port you will need to change the unit card fetch from the frontend right now it only looks for 8090

Also on the update/Patchserver side of things right now if you wish to push an update you must change the number inside the PATCHSERVER/game/ButterVersion.txt
^^as long as this number is different from the client it will push the update to the client. (note you currently cant skip updates but I will implement that once I have time.)
^^this also checks the server ip too so you dont have to worry about joining another server with the same ButterVersion.txt.
^^the Patchserver keeps all your original files before the patch and just renames the old ones to .butterold so when you change server or update it will restore all those and delete the old ones from the recent patch like a fresh wipe before update each time. <--needs some improvment though
^^lastly the game folder under PATCHSERVER is 1:1 of the client folder so make sure the layout is the same I included an empty dat folder as well.

If dev mode doesnt launch/ instantly closes after running the command go into the src-tauri folder and delete the `ButterClient` folder. 
^^its cause of the manifest and the server checking, I prob goofed somewhere for this so prob an easy fix I just didnt priotize it. soz again.

Also one last thing as a lil tip, I create a Client folder inside src-tauri and I put my dat and dll files in there then point the dev mode at it for further testing
^^this is optional but its what I do which helps slightly also it acts as a backup for the game files too.