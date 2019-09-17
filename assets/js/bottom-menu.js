function toggleMainMenu() {
    var mainMenu = document.getElementById("main-menu-id");
    mainMenu.classList.toggle("none");

    var unitMenu = document.getElementById("unit-menu-id");
    unitMenu.classList.add("none");
}

function toggleUnitMenu() {
    var unitMenu = document.getElementById("unit-menu-id");
    unitMenu.classList.toggle("none");

    var mainMenu = document.getElementById("main-menu-id");
    mainMenu.classList.add("none");
}

function toggleAllMenusOff() {
    var unitMenu = document.getElementById("unit-menu-id");
    unitMenu.classList.add("none");

    var mainMenu = document.getElementById("main-menu-id");
    mainMenu.classList.add("none");
}