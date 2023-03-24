# GitHub Issue Notifier

[Deploy this function on flows.network](#deploy-github-issue-notifier-app-on-your-slack-channel), and you will get a Salck bot that sending you a Slack message when a GitHub issue labled with `good first issue`, `help wanted`, and `LFX Mentorship`. You can also change the lables according to your needs.

<img width="529" alt="image" src="https://user-images.githubusercontent.com/45785633/227550918-f1da6560-7ac0-4a2d-9f47-23e386a08a61.png">

## Deploy GitHub issue notifier on your Slack channel

To install the GitHub issue notifier, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/github-issue-notifer/) and customize the code based on your needs. 

<img width="683" alt="image" src="https://user-images.githubusercontent.com/45785633/227545639-661b9928-d484-482f-aaef-f1eca2b8370e.png">
<img width="756" alt="image" src="https://user-images.githubusercontent.com/45785633/227546249-1b5425d1-454d-4aac-aa6b-114b2aaf087b.png">

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the ChatGPT GitHub APP
3. Authenticate the [flows.network](https://flows.network/) to access the `github-issue-notifier` repo you just forked. 
<img width="883" alt="image" src="https://user-images.githubusercontent.com/45785633/227555428-deccd96c-bbef-4828-9f21-0fa41bfee675.png">


4. click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow. Here we can see, we need to configue two SaaS integrations.

<img width="931" alt="image" src="https://user-images.githubusercontent.com/45785633/227555540-1828b90f-410e-4bba-a789-529a1729af02.png">

1. Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo. This repo is the one you changed in your code above.

2. Click the "Connect/+ Add new authentication" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo. This repo is the one you changed in your code above.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the GitHub issue notifier App goes live. Go ahead and let the community know what theu can help!

<img width="1116" alt="image" src="https://user-images.githubusercontent.com/45785633/227556377-44ebee6b-e64c-4a88-ab41-b6514721691e.png">

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!








