try {
    var b = typeof browser === undefined ? chrome : browser;
} catch (e) {
    var b = chrome;
}


b.pageAction.onClicked.addListener(async () => {
    let tab = await browser.tabs.query({active: true, currentWindow: true});

    if (tab[0] === undefined) {
        return;
    }
    tab = tab[0];

    fetch("https://cheman.org/api/addPost", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
            url: tab.url,
            title: tab.title,
            favicon_url: tab.favIconUrl || null,
            kk: 'Oopai4aiphaiZ7shang4Tu5i'
            // todo add tab.faviconUrl
        })
    });
});

