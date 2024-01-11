<script lang="ts">
    type Album = {
        id: number;
        path: string;
    };

    type Tier = {
        name: string;
        color: string;
        albums: Album[];
    };

    const tiers: Tier[] = [
        { name: "S", color: "Red", albums: [] },
        { name: "A", color: "Orange", albums: [] },
        { name: "B", color: "Yellow", albums: [] },
        { name: "C", color: "Green", albums: [] },
        { name: "D", color: "Blue", albums: [] },
    ];

    let selAlbum: Album | undefined;

    const albums: Album[] = [
        {
            id: 0,
            path: "./src/lib/images/amnesiac.png",
        },
        {
            id: 1,
            path: "./src/lib/images/angles.jpg",
        },
        {
            id: 2,
            path: "./src/lib/images/bends.png",
        },
        {
            id: 3,
            path: "./src/lib/images/comedownmachine.jpg",
        },
        {
            id: 4,
            path: "./src/lib/images/firstimpressionsofearth.jpg",
        },
        {
            id: 5,
            path: "./src/lib/images/hailtothethief.png",
        },
        {
            id: 6,
            path: "./src/lib/images/inrainbows.png",
        },
        {
            id: 7,
            path: "./src/lib/images/isthisit.png",
        },
        {
            id: 8,
            path: "./src/lib/images/kida.png",
        },
        {
            id: 9,
            path: "./src/lib/images/kingoflimbs.png",
        },
        {
            id: 10,
            path: "./src/lib/images/moonshapedpool.png",
        },
        {
            id: 11,
            path: "./src/lib/images/newabnormal.jpg",
        },
        {
            id: 12,
            path: "./src/lib/images/okcomputer.png",
        },
        {
            id: 13,
            path: "./src/lib/images/pablohoney.png",
        },
        {
            id: 14,
            path: "./src/lib/images/roomonfire.png",
        },
    ];

    let selState = 0;
    let selIndex = 0;
    let selTier = 0;

    const move = (e: KeyboardEvent) => {
        if (selState === 1) {
            switch (e.key) {
                case "k":
                    if (selTier > 0) {
                        tiers[selTier].albums = [
                            ...tiers[selTier].albums.slice(
                                0,
                                tiers[selTier--].albums.length - 1
                            ),
                        ];
                        if (selAlbum) {
                            tiers[selTier].albums = [
                                ...tiers[selTier].albums,
                                selAlbum,
                            ];
                        }
                    }
                    break;
                case "j":
                    if (selTier < 4) {
                        tiers[selTier].albums = [
                            ...tiers[selTier].albums.slice(
                                0,
                                tiers[selTier++].albums.length - 1
                            ),
                        ];
                        if (selAlbum) {
                            tiers[selTier].albums = [
                                ...tiers[selTier].albums,
                                selAlbum,
                            ];
                        }
                    }
                    break;
                case "Enter":
                    if (albums.length === 0) {
                        selState = 2;
                        break;
                    } else {
                        selState = 0;
                        selIndex = 0;
                        selTier = 0;
                    }
                    break;
                default:
                    break;
            }
        } else if (selState === 0) {
            switch (e.key) {
                case "Enter":
                    selState = 1;

                    selAlbum = albums[selIndex];
                    if (selAlbum) {
                        albums.splice(albums.indexOf(selAlbum), 1);
                        tiers[0].albums = [...tiers[0].albums, selAlbum];
                    }

                    selIndex = -1;
                    break;
                case "h":
                    if (selIndex > 0) {
                        selIndex--;
                    }
                    break;
                case "j":
                    if (selIndex + 8 <= albums.length - 1) selIndex += 8;
                    break;
                case "k":
                    if (selIndex - 8 >= 0) {
                        selIndex -= 8;
                    }
                    break;
                case "l":
                    if (selIndex + 1 < albums.length) {
                        selIndex++;
                    }
                    break;
                default:
                    break;
            }
        } else {
        }
    };

    function match(album: Album, _: number): boolean {
        return albums.indexOf(album) === selIndex;
    }
</script>

<div class="page">
    <div class="container">
        <div class="title">Album Tier List</div>
        <div class="tiers">
            {#each tiers as tier, index (tier.name)}
                <div class="row">
                    <div class="tier {tier.color} ">
                        <span class="tiername">{tier.name}</span>
                    </div>
                    <div class="tierAlbums">
                        {#each tiers[index].albums as album (album.id)}
                            <img src={album.path} alt="" class="tierAlbum" />
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    </div>
    <div class="albums {selState === 0 ? 'pickAlbum' : ''}">
        {#each albums as album (album.id)}
            <img
                src={album.path}
                alt=""
                class="albumsImg {match(album, selIndex) ? 'selAlbum' : ''}"
            />
            <!-- have to pass selIndex into the match function as that is the value that changes, otherwise the DOM rerender won't be triggered-->
        {/each}
    </div>
</div>

<svelte:window on:keydown={move} />

<style>
    .page {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        width: 100%;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        align-items: center;
        width: 70%;
    }

    .title {
        font-size: xx-large;
        color: white;
        padding: 2% 0;
    }

    .tiers {
        width: 100%;
    }

    .row {
        display: grid;
        grid-template-columns: 10% 90%;
        background-color: rgb(41, 40, 40);
        padding: 0.1% 0;
        border-top: 1px solid black;
        border-bottom: 1px solid black;
    }

    .tier {
        padding: 30%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .tiername {
        font-size: x-large;
    }

    .tierAlbums {
        display: flex;
        flex-direction: row;
        align-items: center;
        height: 100%;
        background-color: rgb(41, 40, 40);
        overflow-x: auto;
    }

    .tierAlbums::-webkit-scrollbar {
        display: none;
    }

    .tierAlbum {
        height: 93%;
        width: 10%;
        padding-left: 1%;
    }

    .albums {
        margin: 2.5% 0;
        display: grid;
        grid-template-columns: repeat(8, 12.69%);
        width: 60%;
        padding: 1%;
        background-color: rgb(41, 40, 40);
    }

    .albumsImg {
        width: 80%;
        height: 80%;
        border: 4px solid rgb(41, 40, 40);
    }

    .pickAlbum {
        border: 1px solid green;
    }

    .selAlbum {
        border: 4px solid white;
    }

    .Red {
        background-color: rgb(245, 80, 80);
        border: 5px solid rgb(245, 80, 80);
    }

    .Orange {
        background-color: rgb(255, 154, 71);
        border: 5px solid rgb(255, 154, 71);
    }

    .Yellow {
        background-color: rgb(236, 236, 85);
        border: 5px solid rgb(236, 236, 85);
    }

    .Green {
        background-color: greenyellow;
        border: 5px solid greenyellow;
    }

    .Blue {
        background-color: cyan;
        border: 5px solid cyan;
    }
</style>
