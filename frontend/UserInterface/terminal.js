module.exports  = {
  input:"x",
  output:"x",
  terminal: function() {

    // command map and commands
    var commandMap = new Map(
      [
        ["print", user_print_command],
      ]
    )

    function user_print_command(args){
      str = ""
      for(arg in args){
          str += args[arg] + " ";
      }
      log(str);

    }

    // private variables
    var output="";
    var input="";
    var data=[] // reference to the data


    //private functions
     function colouredText(msg, colour) {
      return "<text class='"+colour+"'>"+msg+"</text>";
    }

    function updateScroll() {
      terminal.scrollTop = terminal.scrollHeight;
    }

    function log(msg, newline=true) {
        output += colouredText("WeLD: ","green") + msg + (newline?"<br/>":"") ;
        document.getElementById("terminal").innerHTML = output + "UserIn: " + input + "<";
        updateScroll()
      };

    function logError(msg, newline=true){
        output += colouredText("WeLD (error): ","red") + msg + (newline?"<br/>":"");
        document.getElementById("terminal").innerHTML = output + "UserIn: " + input + "<";
        updateScroll()
      }

    function logWarning(msg, newline=true){
        output += colouredText("WeLD (warning): ","orange") + msg + (newline?"<br/>":"");
        document.getElementById("terminal").innerHTML = output + "UserIn: " + input + "<";
        updateScroll()
      }

    // public functions
    this.logWarning = function(msg,newline=true){logWarning(msg,newline)}
    this.logError = function(msg,newline=true){logError(msg,newline)}
    this.log = function(msg,newline=true){log(msg,newline)}
    this.colouredText = function(msg, colour){
      return colouredText(msg, colour);
    }
    this.setData = function(d){data=d}


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
        terminal.style.color = "white";
        terminal.style.backgroundColor = "black";
        terminal.innerHTML = output + "UserIn: " + input + "<";
    })

    terminal.addEventListener("focusout",function(event){
        terminal.style.backgroundColor = "rgb(20,20,20)";
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
