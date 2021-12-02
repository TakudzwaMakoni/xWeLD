module.exports  = {
  input:"x",
  output:"x",
  terminal: function() {

    // private variables
    var output="";
    var input="";
    terminal = document.createElement("div");
    document.body.appendChild(terminal);
    terminal.setAttribute("tabindex","0");
    terminal.setAttribute("id","terminal");
    terminal.innerHTML = output + "User: " + input + "<";
    terminal.style.position = "fixed";
    terminal.style.top = "2.5%";
    terminal.style.right = "1.5%";
    terminal.style.width = "25%";
    terminal.style.height = "25%";
    terminal.style.color = "rgb(173,172,173)";
    terminal.style.padding = "2.5px";
    terminal.style.fontFamily = "monospace";
    terminal.style.overflowX = "scroll";
    terminal.style.overflowY = "scroll";
    terminal.addEventListener("focus",function(event){
        terminal.style.color = "black";
        terminal.innerHTML = output + "UserIn: " + input + "<";
    })
    terminal.addEventListener("focusout",function(event){
        terminal.style.backgroundColor = "white";
        terminal.style.color = "rgb(173,172,173)";
    });

    terminal.focus();
    terminal.addEventListener("keydown", function( event ) {
      var key = event.keyCode;
      var char = String.fromCharCode((96 <= key && key <= 105) ? key-48 : key).toLowerCase();
      // If the user has pressed enter
      switch (key) {
        case 32 /*space*/:
        {
          input+=(input.slice(-1)===" "?"":" ");
          terminal.innerHTML = output + "UserIn: " + input + "<";;
          break;
        }
        case 13 /*enter*/:
        {
          var args = input.split(" ");
          var command = args[0]
          console.log(args)
          /*
          if(!commandMap.has(command))
          {
            input = "";
            logError("no such command.")
            break;
          } else {
            var fn = commandMap.get(command);
            fn(args.slice(1));
            input = "";
            terminal.innerHTML = output + "UserIn: " +input + "<";
            break;
          }
          */
        }
        case 8/*backspace*/:
        {
          if(input.length>0)
          {
            input = input.slice(0, -1);
            terminal.innerHTML = output + "UserIn: " +input + "<";
          }
          break;
        }
        case 37/*left*/:
        {

          break;
        }
        case 39/*right*/:
        {

          break;
        }
          case 38/*up*/:
        {

          break;
        }
          case 40/*down*/:

            break;
        default:
        {
          input += char;
          terminal.innerHTML = output + "UserIn: " + input + "<";
        }

      }
    }, false);
    return this;
  }
}
