# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.16

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/louis/rust-sx12xx/sx12xx-sys/sx12xx

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/louis/rust-sx12xx/sx12xx-sys/sx12xx

# Include any dependencies generated for this target.
include CMakeFiles/sx12xx.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/sx12xx.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/sx12xx.dir/flags.make

CMakeFiles/sx12xx.dir/sx12xx.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx12xx.c.o: sx12xx.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/sx12xx.dir/sx12xx.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx12xx.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx12xx.c

CMakeFiles/sx12xx.dir/sx12xx.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx12xx.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx12xx.c > CMakeFiles/sx12xx.dir/sx12xx.c.i

CMakeFiles/sx12xx.dir/sx12xx.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx12xx.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx12xx.c -o CMakeFiles/sx12xx.dir/sx12xx.c.s

CMakeFiles/sx12xx.dir/board.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/board.c.o: board.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building C object CMakeFiles/sx12xx.dir/board.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/board.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/board.c

CMakeFiles/sx12xx.dir/board.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/board.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/board.c > CMakeFiles/sx12xx.dir/board.c.i

CMakeFiles/sx12xx.dir/board.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/board.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/board.c -o CMakeFiles/sx12xx.dir/board.c.s

CMakeFiles/sx12xx.dir/sx126x/radio.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx126x/radio.c.o: sx126x/radio.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building C object CMakeFiles/sx12xx.dir/sx126x/radio.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx126x/radio.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/radio.c

CMakeFiles/sx12xx.dir/sx126x/radio.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx126x/radio.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/radio.c > CMakeFiles/sx12xx.dir/sx126x/radio.c.i

CMakeFiles/sx12xx.dir/sx126x/radio.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx126x/radio.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/radio.c -o CMakeFiles/sx12xx.dir/sx126x/radio.c.s

CMakeFiles/sx12xx.dir/sx126x/sx126x.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx126x/sx126x.c.o: sx126x/sx126x.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building C object CMakeFiles/sx12xx.dir/sx126x/sx126x.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx126x/sx126x.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/sx126x.c

CMakeFiles/sx12xx.dir/sx126x/sx126x.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx126x/sx126x.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/sx126x.c > CMakeFiles/sx12xx.dir/sx126x/sx126x.c.i

CMakeFiles/sx12xx.dir/sx126x/sx126x.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx126x/sx126x.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/sx126x.c -o CMakeFiles/sx12xx.dir/sx126x/sx126x.c.s

CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.o: sx126x/sx126x-board.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building C object CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/sx126x-board.c

CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/sx126x-board.c > CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.i

CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx126x/sx126x-board.c -o CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.s

CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.o: sx1272/sx1272-board.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Building C object CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1272/sx1272-board.c

CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1272/sx1272-board.c > CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.i

CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1272/sx1272-board.c -o CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.s

CMakeFiles/sx12xx.dir/sx1272/sx1272.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx1272/sx1272.c.o: sx1272/sx1272.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_7) "Building C object CMakeFiles/sx12xx.dir/sx1272/sx1272.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx1272/sx1272.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1272/sx1272.c

CMakeFiles/sx12xx.dir/sx1272/sx1272.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx1272/sx1272.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1272/sx1272.c > CMakeFiles/sx12xx.dir/sx1272/sx1272.c.i

CMakeFiles/sx12xx.dir/sx1272/sx1272.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx1272/sx1272.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1272/sx1272.c -o CMakeFiles/sx12xx.dir/sx1272/sx1272.c.s

CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.o: sx1276/sx1276-board.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_8) "Building C object CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1276/sx1276-board.c

CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1276/sx1276-board.c > CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.i

CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1276/sx1276-board.c -o CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.s

CMakeFiles/sx12xx.dir/sx1276/sx1276.c.o: CMakeFiles/sx12xx.dir/flags.make
CMakeFiles/sx12xx.dir/sx1276/sx1276.c.o: sx1276/sx1276.c
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_9) "Building C object CMakeFiles/sx12xx.dir/sx1276/sx1276.c.o"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -o CMakeFiles/sx12xx.dir/sx1276/sx1276.c.o   -c /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1276/sx1276.c

CMakeFiles/sx12xx.dir/sx1276/sx1276.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/sx12xx.dir/sx1276/sx1276.c.i"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1276/sx1276.c > CMakeFiles/sx12xx.dir/sx1276/sx1276.c.i

CMakeFiles/sx12xx.dir/sx1276/sx1276.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/sx12xx.dir/sx1276/sx1276.c.s"
	/usr/bin/arm-none-eabi-gcc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/sx1276/sx1276.c -o CMakeFiles/sx12xx.dir/sx1276/sx1276.c.s

# Object files for target sx12xx
sx12xx_OBJECTS = \
"CMakeFiles/sx12xx.dir/sx12xx.c.o" \
"CMakeFiles/sx12xx.dir/board.c.o" \
"CMakeFiles/sx12xx.dir/sx126x/radio.c.o" \
"CMakeFiles/sx12xx.dir/sx126x/sx126x.c.o" \
"CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.o" \
"CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.o" \
"CMakeFiles/sx12xx.dir/sx1272/sx1272.c.o" \
"CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.o" \
"CMakeFiles/sx12xx.dir/sx1276/sx1276.c.o"

# External object files for target sx12xx
sx12xx_EXTERNAL_OBJECTS =

libsx12xx.a: CMakeFiles/sx12xx.dir/sx12xx.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/board.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx126x/radio.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx126x/sx126x.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx126x/sx126x-board.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx1272/sx1272-board.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx1272/sx1272.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx1276/sx1276-board.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/sx1276/sx1276.c.o
libsx12xx.a: CMakeFiles/sx12xx.dir/build.make
libsx12xx.a: CMakeFiles/sx12xx.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles --progress-num=$(CMAKE_PROGRESS_10) "Linking C static library libsx12xx.a"
	$(CMAKE_COMMAND) -P CMakeFiles/sx12xx.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/sx12xx.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/sx12xx.dir/build: libsx12xx.a

.PHONY : CMakeFiles/sx12xx.dir/build

CMakeFiles/sx12xx.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/sx12xx.dir/cmake_clean.cmake
.PHONY : CMakeFiles/sx12xx.dir/clean

CMakeFiles/sx12xx.dir/depend:
	cd /home/louis/rust-sx12xx/sx12xx-sys/sx12xx && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/louis/rust-sx12xx/sx12xx-sys/sx12xx /home/louis/rust-sx12xx/sx12xx-sys/sx12xx /home/louis/rust-sx12xx/sx12xx-sys/sx12xx /home/louis/rust-sx12xx/sx12xx-sys/sx12xx /home/louis/rust-sx12xx/sx12xx-sys/sx12xx/CMakeFiles/sx12xx.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/sx12xx.dir/depend

