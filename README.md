# Mini-windmill

## Execute Go programs

The intention behind this project is to teach myself rust and little bit of Go.

What it does is basically provide an interface to store Go programs remotely and run them either individually or as part of a flow/sequence.

The backend is **Rust** with **MongoDB** and in frontend is **Svelte**.

**Note 1:** This project might seem random. But initially the goal here was to chain multiple Go programs, have them interact with external services like Airtable, and automate some workflows. Currently it's doing none of that.

**Note 2:** This was inspired by [Windmill](https://github.com/windmill-labs/windmill), hence the name.

**Note 3:** Demo is hosted here: https://mini-windmill.vercel.app/

![image info](images/step.png)

<br/>

**Flow** is a list of steps run in a sequence.

![image info](images/flow.png)

<br/>

**Stat** shows the state of the 3 worker threads and the output of a job execution. (Clicking on hash would show the output)
![image info](images/stat.png)

