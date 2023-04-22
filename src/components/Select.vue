<template>
    <div class="select-container">
        <div class="select-item">
            <img class="select-img" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADYAAAA2CAMAAAC7m5rvAAAAkFBMVEUAAAB5eXl5eXl4eHh5eXl6enp4eHj////z8/OXl5f5+fn9/f2Ghob7+/t8fHyamprl5eXh4eHa2trKysrFxcWrq6ulpaWLi4uCgoLv7+/e3t7W1tbS0tLCwsK9vb26urqzs7Ourq6UlJR/f3/19fXs7Ozp6enPz8+hoaGNjY3MzMympqaenp6dnZ2RkZGPj49I5iYVAAAABnRSTlMA88i9ZRljrLGUAAABc0lEQVRIx52W6XLCMAwGlRCsECDc4SzlKpSe7/92FSk0lE8G2/sjzDAswcabEQlJHBlnojihE/Wa8aRWF00sb48oMQEkFIdoMUUhWkTGyjDrjo0Fu1Ywc99f67Lw5at98omFrzYrtebYT9vwLwM/bX7W2i0f7dgUZZnJ5clHexMhzYdyzbbuWqstwvP5xV0rbzMx5lDe1FWbXBb1fVriylVbllt42dCpo7bd/f1hDRZGbtqqPB7VYdk7afn06jB+sPDqoo1Y2FyH0HPRXv6F9s5C57FWVJ/Db0HN9qvWWq6k5rlsVHQyZj480vqMYK4EeeoM7msLENRcSclzl97QhFxJy1PtaGvVoEvr26TlCUCupOQJYK6k5AlgroR5IpgrYZ4I5kqYJ4K5kiVPWxio7bWwLLmSJU89+jlqPe2hcZvrEbSsWrNKnjJzAdqAeaYL1e3SCW5JMcrNXTrrFqzNg3AtcHgKHNUCB8OwMTR06A0csX8ApgxQwSDFVWYAAAAASUVORK5CYII=" alt="">
            <span class="select-text">解释</span>
        </div>
        <div class="select-item">
            <img class="select-img" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADYAAAA2CAMAAAC7m5rvAAAAkFBMVEUAAAB5eXl5eXl4eHh5eXl6enp4eHj////z8/OXl5f5+fn9/f2Ghob7+/t8fHyamprl5eXh4eHa2trKysrFxcWrq6ulpaWLi4uCgoLv7+/e3t7W1tbS0tLCwsK9vb26urqzs7Ourq6UlJR/f3/19fXs7Ozp6enPz8+hoaGNjY3MzMympqaenp6dnZ2RkZGPj49I5iYVAAAABnRSTlMA88i9ZRljrLGUAAABc0lEQVRIx52W6XLCMAwGlRCsECDc4SzlKpSe7/92FSk0lE8G2/sjzDAswcabEQlJHBlnojihE/Wa8aRWF00sb48oMQEkFIdoMUUhWkTGyjDrjo0Fu1Ywc99f67Lw5at98omFrzYrtebYT9vwLwM/bX7W2i0f7dgUZZnJ5clHexMhzYdyzbbuWqstwvP5xV0rbzMx5lDe1FWbXBb1fVriylVbllt42dCpo7bd/f1hDRZGbtqqPB7VYdk7afn06jB+sPDqoo1Y2FyH0HPRXv6F9s5C57FWVJ/Db0HN9qvWWq6k5rlsVHQyZj480vqMYK4EeeoM7msLENRcSclzl97QhFxJy1PtaGvVoEvr26TlCUCupOQJYK6k5AlgroR5IpgrYZ4I5kqYJ4K5kiVPWxio7bWwLLmSJU89+jlqPe2hcZvrEbSsWrNKnjJzAdqAeaYL1e3SCW5JMcrNXTrrFqzNg3AtcHgKHNUCB8OwMTR06A0csX8ApgxQwSDFVWYAAAAASUVORK5CYII=" alt="">
            <span class="select-text">润色</span>
        </div>
        <div class="down-row"></div>
        <div class="select-item">
            <img class="select-img" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADYAAAA2CAMAAAC7m5rvAAAAkFBMVEUAAAB5eXl5eXl4eHh5eXl6enp4eHj////z8/OXl5f5+fn9/f2Ghob7+/t8fHyamprl5eXh4eHa2trKysrFxcWrq6ulpaWLi4uCgoLv7+/e3t7W1tbS0tLCwsK9vb26urqzs7Ourq6UlJR/f3/19fXs7Ozp6enPz8+hoaGNjY3MzMympqaenp6dnZ2RkZGPj49I5iYVAAAABnRSTlMA88i9ZRljrLGUAAABc0lEQVRIx52W6XLCMAwGlRCsECDc4SzlKpSe7/92FSk0lE8G2/sjzDAswcabEQlJHBlnojihE/Wa8aRWF00sb48oMQEkFIdoMUUhWkTGyjDrjo0Fu1Ywc99f67Lw5at98omFrzYrtebYT9vwLwM/bX7W2i0f7dgUZZnJ5clHexMhzYdyzbbuWqstwvP5xV0rbzMx5lDe1FWbXBb1fVriylVbllt42dCpo7bd/f1hDRZGbtqqPB7VYdk7afn06jB+sPDqoo1Y2FyH0HPRXv6F9s5C57FWVJ/Db0HN9qvWWq6k5rlsVHQyZj480vqMYK4EeeoM7msLENRcSclzl97QhFxJy1PtaGvVoEvr26TlCUCupOQJYK6k5AlgroR5IpgrYZ4I5kqYJ4K5kiVPWxio7bWwLLmSJU89+jlqPe2hcZvrEbSsWrNKnjJzAdqAeaYL1e3SCW5JMcrNXTrrFqzNg3AtcHgKHNUCB8OwMTR06A0csX8ApgxQwSDFVWYAAAAASUVORK5CYII=" alt="">
            <span class="select-text">提问</span>
        </div>
        <div class="select-item">
            <img class="select-img" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADYAAAA2CAMAAAC7m5rvAAAAkFBMVEUAAAB5eXl5eXl4eHh5eXl6enp4eHj////z8/OXl5f5+fn9/f2Ghob7+/t8fHyamprl5eXh4eHa2trKysrFxcWrq6ulpaWLi4uCgoLv7+/e3t7W1tbS0tLCwsK9vb26urqzs7Ourq6UlJR/f3/19fXs7Ozp6enPz8+hoaGNjY3MzMympqaenp6dnZ2RkZGPj49I5iYVAAAABnRSTlMA88i9ZRljrLGUAAABc0lEQVRIx52W6XLCMAwGlRCsECDc4SzlKpSe7/92FSk0lE8G2/sjzDAswcabEQlJHBlnojihE/Wa8aRWF00sb48oMQEkFIdoMUUhWkTGyjDrjo0Fu1Ywc99f67Lw5at98omFrzYrtebYT9vwLwM/bX7W2i0f7dgUZZnJ5clHexMhzYdyzbbuWqstwvP5xV0rbzMx5lDe1FWbXBb1fVriylVbllt42dCpo7bd/f1hDRZGbtqqPB7VYdk7afn06jB+sPDqoo1Y2FyH0HPRXv6F9s5C57FWVJ/Db0HN9qvWWq6k5rlsVHQyZj480vqMYK4EeeoM7msLENRcSclzl97QhFxJy1PtaGvVoEvr26TlCUCupOQJYK6k5AlgroR5IpgrYZ4I5kqYJ4K5kiVPWxio7bWwLLmSJU89+jlqPe2hcZvrEbSsWrNKnjJzAdqAeaYL1e3SCW5JMcrNXTrrFqzNg3AtcHgKHNUCB8OwMTR06A0csX8ApgxQwSDFVWYAAAAASUVORK5CYII=" alt="">
            <span class="select-text">复制</span>
        </div>
    </div>
</template>

    
<script lang="ts" setup>
</script>

<style scoped>
.select-container {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    justify-content: space-around;
    align-items: center;
    border-radius: 8px;
    width: 100%;
    position: absolute;      
      background: #484848;
}

.select-item {
    width: 58px;
    height: 64px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    position: relative;
    border-radius: 8px;
    text-align: center;
    
}

.select-item:hover {
    background: #353535;
}

.down-row {
    opacity: 1;
    position: absolute;
    width: 29px;
    height: 9px;
    bottom: -9px;
    background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADkAAAASCAYAAADhc3ZwAAAAAXNSR0IArs4c6QAAAERlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAA6ABAAMAAAABAAEAAKACAAQAAAABAAAAOaADAAQAAAABAAAAEgAAAAABgJS1AAACZklEQVRYCdVWS0gbURR1Jh8waZqoIQEhIG7LKO6tuFHMQpNJSFGycqebUmhduAu4FiyhKFmEWRg3YfJBAxYKXZR2l0WwXbhwJagIolhSW5JJPG8x8BSdzOeF6oNh7rufc+95901yuR6sZDL5sl6vL0J82263h4nuuS6O425Qu8zzfMHv93/OZDJ/OJpMKpXia7XaLIi+wzNJ256BfAli66hzq1gsXtD13iFJG2KxmKgoyiZ0QVr/FGV0bxvPe5A7f6i+R0kSZ1EUB9DRNJ6Fh4KfgO7QZrMtFwqFr1q1aJJUA6PRaByyBLIvVN3/fqNzUigUWkqn0/861aKLJAGJx+NjzWZzD+JgJ9Bu2kFOAf6HUqm0oTePbpIEMBKJhJCkgo4KehMw9ru02+1vZFn+YgSXN+JcLpeP3W73OIh+MxLHyPcEBF8bJUhyGyJJAnK53LXH4wlD1PzYiS+rhUM9cjqd4yD4ywymoetKJ0gkEr2NRqOMqztF61nLIHjgcDim8/n8mVlsw51UEyHpjc/nm8O+oupYv0HwBzAnrBAkNZkmSYIlSfobDAZFFCOTPeO1HwgEpvAremUV12YVoFqttgRBkFut1jCwRqzikXgc2jYObx5zZ8f/QD35TH+T98HxbXIYGlZR4BpkUzcEsQ3grqB7H+/jW9kzI6kWgVFwBl3dwb5P1el5g+ApBuwERrTvevyN+Jg6ca0EGJL3YR9F0XktP8rWhPzJ5XK96gZBkod5J6niyYQkgOwydGFc4SHaBv1P7Heh3yRDBm1jLXeVJF0s/lf7cY0D0Cler/csm83+pu3dlG8BBYC58B/A2JYAAAAASUVORK5CYII=) repeat-x 50%/auto 9px;
}

.select-img {
    width: 20px;
    margin-bottom: 5px;
}

.select-text {
    display: block;
    text-align: center;
    line-height: 16px;
    font-size: 12px;
    font-weight: 500;
    transform: scale(.84);
    -webkit-font-smoothing: antialiased;
    -webkit-text-stroke-width: 0.2px;
    color: #fff;
}


</style>
