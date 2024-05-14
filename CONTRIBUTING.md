# How to use `git` with this project

### **Step 0 -** `git clone` the repository

*You only need to do this the first time.*

```bash
$ git clone https://github.com/James-Millan/filters.git
$ cd filters
```

### **Step 1 -** Move to the [`dev`](https://github.com/NPCMS/ElementalExplorers/tree/dev) branch

```bash
$ git checkout -b dev origin/dev
```

### **Step 2 -** Pull the latest changes

```bash
$ git pull --ff-only
```
Don't omit the `--ff-only` flag, as it may cause a messy git history.

### **Step 3 -** Branch off [`dev`](https://github.com/James-Millan/filters/tree/dev)

```bash
$ git checkout -b <your-branch-name-here> dev
```

### **Step 5 -** DO NOT COMMIT ALL CHANGES


When committing changes, follow the principle of **atomic commits**. Make each commit into a *single logical unit*. 
The changes made with each commit should be as minimal as possible in order to get the change working.  
Avoid the use of the following command before comitting changes:  
```bash
$ git add .
```
Instead, you should specifically add the files you want to commit to the repository. Failure to comply with this simple rule will result in merge conflicts which are time consuming to resolve.

```bash
# -- after making changes to N files --
$ git add <file1> <file2> ... <fileN>
$ git commit -m "describe change" 
# -- importantly there is no -a flag in the commit here --
$ git push --set-upstream origin/<your-branch-name-here>
```

*You only need the `--set-upstream origin/<your-branch-name-here>` flag on your first commit.*

### **Step 6 -** Open a [pull request](https://github.com/James-Millan/filters/compare) to incorporate changes

Make sure to set the base to `dev`, and compare to the branch you created.

Wait for someone else to review your code and confirm your pull request, as there may be merge conflicts or other issues with your code that need to be fixed before it is included.


### Gratitude

We thank you for contributing to this open source project. Even if you do not want write code to improve it, we encourage you to create issues on the project when you find something you would like to be changed. We express our gratitude to those that have contributed to this project.
