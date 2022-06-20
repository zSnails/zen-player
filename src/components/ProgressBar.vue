<template>
    <div id="main-progress-bar" class="progress-bar" :style="getProgress">
    </div>
</template>
<script>
export default {
    name: "ProgressBar",
    props: {
        progress: {
            type: Number,
            default: 0,
        },
        max: {
            type: Number,
            default: 0,
        }
    },
    computed: {
        getProgress() {
            return {
                '--progress': `${Math.floor((100 * parseInt(this.progress)) / parseInt(this.max))}%`,
            }
        }
    }
}
</script>
<style>
.progress-bar {
    display: block;
    position: absolute;
    bottom: 0;
    left: 0;
    height: 10px;
    width: var(--progress);
    background-color: #FFF;
}
</style>
