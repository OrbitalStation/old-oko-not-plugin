
# World

A **world** is an independent set of *components*, *entities*, *systems* and *events*.

A **component** is like property of an *entity*.
`Component` is basically a generic word for different unions, enums and structs
(all of which are defined using the `ty` statement).
For example, `Health` is a component of *entities* `Player` and `Enemy`.

An **entity** is a set of different components.
Each component can be deleted of added to an entity(if needed).
For example, `Health`, `Position` and `PlayerGuided` are components of an entity `Player`.
In a similar way, `Health`, `Position` and `AI` are components of an entity `Player`.

A **system** is sort of a *law* that describes the behaviour of the world.
Each system has its own place in *Schedule*.
System can query entities on its call and perform operations
on them.

An **event** is a pointer that something happened and needs to be handled by *callback system(s)*.
There are *builtin* events, which are created by the compiler, e.g. the start/end of the program,
and *user-defined*, which are described and created by the programmer.

# Schedule

A **Schedule** is (unsurprisingly) a schedule of systems' executions.
It contains description of `when do I(world) call this system?`.
Each system, in turn, divides into *staged* and *callback* systems.

## Staged systems

These systems are executed one after another(or in parallel) continuously
throughout the whole life of the world
(unless a system is explicitly finished, i.e. marked as `no longer execute me`)

## Callback systems

These systems are called when some event occurs, for example the program starts,
or user types smth, or hdd finished writing, etc.

# Example(2D game prototype)

                             ------------------
                             |   COMPONENTS   |
                             ------------------
                                     /|\
                    ________________/ | \______________
                    |   POSITION   |  |  |   HEALTH   |
                    ----------------  |  --------------
           fields ----> | vec2 |      |      | u8 |
                        --------      |      ------
                                     / \
           _________________________/   \__________
           |   PLAYER_CONTROLLED   |     |   AI   |
           -------------------------     ----------


                                   ----------------
                                   |   ENTITIES   |
                                   ----------------
                                          / \
                           ______________/   \____________
                           |   PLAYER   |    |   ENEMY   |
                         ----------------    -------------
                    /--> |       health |    | health   |
                    |    ----------------    ------------
        components ----> |     position |    | position |
                    |    ----------------    ------------
                    \--> | playerGuided |    | ai       |
                         ----------------    ------------

                                   ---------------
                                   |   SYSTEMS   |
                                   ---------------
                                         /\
                         _______________/  \_________
                         | FALL SYSTEM |   | ETC... |
                      ------------------   ----------
        queries ----> | query POSITION |
                      ------------------


                                     ----------------
                                     |   SCHEDULE   |
                                     ----------------
                                             |
                                             |
                                             v

                                   ------- fall ------->
