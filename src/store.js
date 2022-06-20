import { reactive } from 'vue';

export const store = reactive({
    _cover: '',
    set cover(new_) {
        this._cover = new_;
    },
    get cover() {
        return this._cover;
    }
});
