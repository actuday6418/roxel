# roxel
Implementation of the voxel space algorithm in Rust, inspired by [this](https://github.com/mcsalgado/voxel_space) repository. 

# Running

```cargo run <color_file> <height_file>```

Eg:

```cargo run image/hut/color.png image/hut/height.png```

![simplescreenrecorder-_4_](https://user-images.githubusercontent.com/56124831/107115734-b0777780-6894-11eb-97f4-37ab75fe04b6.gif)

# Color and height map quickly rendered out of blender
![simplescreenrecorder-_5_](https://user-images.githubusercontent.com/56124831/107121404-3ce76180-68b8-11eb-8433-9da4be63dc43.gif)

# Horizontal lines glitch
I have no idea why I've got all those horizontal lines in the render. My code does not allow for anything other than vertical lines (as specified by the algorithm). So I tried asking around in SFML forums and got no responses. Help if you know what's wrong.
