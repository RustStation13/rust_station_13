## Coding standard

When coding, you are required to adhere to the following standards in order to produce readable and maintainable code

This repository MUST be able to compile to Windows, Linux and OSX.

## The dos:

1. Keep systems separate, and only import what you need to systems.
2. Keep things organized, if no fitting file exists make one.
3. Write tests for your code where possible, you can utilize the Tests folder to place tests in to keep the main files clean.

## The don'ts:

1. Don't name a variable something vauge like x or y, use descriptive names
2. Don't make lines require a ultra widescreen monitor to fit it all on 1 line.
3. Don't initialize systems more than once
4. Don't make your code only run on 1 platform, if you need a platform specific function, ensure to also make it work on other platforms somehow!
5. Don't rely on precompiled binaries, all code must be visible within the repo itself, you are however allowed to bring in other packages if needed.
6. Don't paste unvetted AI-generated code directly into the codebase. Using AI for explanations or drafts is fine—just make sure the final code is clean and maintainable.

## Deploying

Only run this if you are certain you don't need to change anything else. Test your local build thoroughly before you execute the build workflow.

To start a deployment, follow these steps:
1. Do the standard `git commit`, but do not push yet.
2. Run `git tag vMajor.Minor.Fix` (replace Major, Minor, and Fix with the version numbers).
3. Run `git push origin vMajor.Minor.Fix`.

After following these steps, your code will automatically start compiling for all platforms and be sent off for review. Be aware that building release binaries takes a long time, and you should generally avoid doing this. Leave this part to the head maintainers.

## Pull requests:

When making a pull request, adhere to the following:

1. Ensure you have followed the dos and don'ts above.
2. Have a descriptive yet short title.
3. Describe what it is you have done.
4. Include a full changelog of the details.
5. Add one of the head maintainers as a Reviewee.

Missing any of these will result in your PR being rejected on the spot.

After your PR is under review, be sure to check in for any comments any of the maintainers have, respond accordingly to any questions stated.

## Becoming a maintainer:

TLDR:

1. Have at least 5 PR's merged to the main branch
2. Be a generally nice and fun person to work with

What we look for in a maintainer is a person who can produce code that follows the standard we have set forth, and people who are kind and fun to work with. We do not want anyone who openly is rude or disruptive. We also look for active developers, who are willing to spend a bit of time making more than just 1 PR and calling it done.

What you can do as a full maintainer:

1. You are able to review PR's
2. You get access to the exclusive Maintainers chat in the discord.