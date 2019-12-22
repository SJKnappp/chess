#include "main.h"

//prints board state out
void print(board board){
  Colour::Modifier blue(Colour::FG_BLUE);
  Colour::Modifier def(Colour::FG_DEFAULT);

  //prints top line
  char val = 'a';
  val -= 1;
  for(int i = 0; i < 8; i++){
    val += 1;
    std::cout << val << "   ";
  }std::cout << '\n';

  for(int j=0;j<8;j++){
    for(int i=0;i<8;i++){
      switch (board.tiles[i][j].player) {
        case 0:
        std::cout << def;
        break;
        case 1:
        std::cout << blue;
        break;
        case 2:
        std::cout << def;
        break;
      }
      std::cout << board.tiles[i][j].state  << def << " | ";
    }
    //prints row number
    std::cout << " " << j + 1 << '\n';

    for(int i=0;i<31;i++){
      if(i == 2){
        std::cout << '+';
      }else if((i-6) % 4 == 0 ){
        std::cout << '+';
      }else{
        std::cout << '-';
      }
    }
    std::cout << '\n';
  }
}

//creats new board
board intialise(){
  board newboard;
  //black
  newboard.tiles[0][0].state = 'r';
  newboard.tiles[1][0].state = 'n';
  newboard.tiles[2][0].state = 'b';
  newboard.tiles[3][0].state = 'q';
  newboard.tiles[4][0].state = 'k';
  newboard.tiles[5][0].state = 'b';
  newboard.tiles[6][0].state = 'n';
  newboard.tiles[7][0].state = 'r';
  newboard.tiles[0][1].state = 'p';
  newboard.tiles[1][1].state = 'p';
  newboard.tiles[2][1].state = 'p';
  newboard.tiles[3][1].state = 'p';
  newboard.tiles[4][1].state = 'p';
  newboard.tiles[5][1].state = 'p';
  newboard.tiles[6][1].state = 'p';
  newboard.tiles[7][1].state = 'p';
  //white
  newboard.tiles[0][7].state = 'r';
  newboard.tiles[1][7].state = 'n';
  newboard.tiles[2][7].state = 'b';
  newboard.tiles[3][7].state = 'q';
  newboard.tiles[4][7].state = 'k';
  newboard.tiles[5][7].state = 'b';
  newboard.tiles[6][7].state = 'n';
  newboard.tiles[7][7].state = 'r';
  newboard.tiles[0][6].state = 'p';
  newboard.tiles[1][6].state = 'p';
  newboard.tiles[2][6].state = 'p';
  newboard.tiles[3][6].state = 'p';
  newboard.tiles[4][6].state = 'p';
  newboard.tiles[5][6].state = 'p';
  newboard.tiles[6][6].state = 'p';
  newboard.tiles[7][6].state = 'p';

  for(int i =0; i < 8; i++){
    for(int j = 0; j < 2; j++){
      newboard.tiles[i][j].player = 1;
    }
    for(int j =6; j <8;j++){
      newboard.tiles[i][j].player = 2;
    }
  }
  return newboard;
}

//checks move and intial and final position
move allowedMove(board board, move input, bool isWhite){
  //moving pawns

  move output = {}; //outputs input location if move not allowed and peice to move if allowed
  int state;

  output.state = 'a'; //used to compair in next function

  if(isWhite == 0){
    state =1;
  }else{state=2;}

  if(input.x < 0 || input.x > 7 || input.y < 0 || input.y > 7){
    return input;
  }

  if(state == board.tiles[input.x][input.y].player){
    return input;
  }

  switch (input.state) {
    case 'p':{
      if(isWhite == 1){
        std::cout << input.x << input.y << '\n';
        if(board.tiles[input.x][input.y+1].state == 'p'){
          board.tiles[input.x][input.y] = board.tiles[input.x][input.y+1];
          output.x = input.x;
          output.y = input.y+1;
          return output;
        }
      }else{
        if(board.tiles[input.x][input.y-1].state == 'p'){
          board.tiles[input.x][input.y] = board.tiles[input.x][input.y-1];
          output.x = input.x;
          output.y = input.y-1;
          return output;
        }
      }break;}
      case 'r':{
        bool testUp = 1, testDown = 1, testRight =1, testLeft =1;

          for(int i=0; i<7; i++){
            if((board.tiles[input.x][input.y+i].state == 'r' || board.tiles[input.x][input.y+i].state == ' ') && testUp == 1 && input.y+i < 8){
              if(board.tiles[input.x][input.y+i].state == 'r') { output.x = input.x; output.y = (input.y+i); return output;}
            }else{testUp = 0; }
            if((board.tiles[input.x][input.y-i].state == 'r' || board.tiles[input.x][input.y-i].state == ' ') && testDown == 1 && input.y+i > 0){
              if(board.tiles[input.x][input.y-i].state == 'r'){output.x = input.x; output.y = (input.y-i); return output;}
            }else{testDown = 0;}
            if((board.tiles[input.x+i][input.y].state == 'r' || board.tiles[input.x+i][input.y].state == ' ') && testRight == 1 && input.x+i < 8){
              if(board.tiles[input.x+i][input.y].state == 'r'){output.x = input.x+i; output.y = input.y; return output;}
            }else{testRight = 0;}
            if((board.tiles[input.x-i][input.y].state == 'r' || board.tiles[input.x-i][input.y].state == ' ') && testLeft == 1 && input.x-i >0){
              if(board.tiles[input.x-i][input.y].state == 'r'){output.x = input.x-i; output.y = input.y; return output;}
            }else{testLeft = 0;}

            if(testUp==0 && testDown == 0 && testLeft == 0 && testRight ==0){
              return input;
            }
        }
        break;
      }
      case 'n':{}
      break;}
      case 'b':{
      break;}
      case 'q':{
      break;}
      case 'k':{
      break;}
    }
    return input;
}

//moves the peices on the board
board movePiece(board board, move start, move end){

  board.tiles[end.x][end.y] = board.tiles[start.x][start.y];
  board.tiles[start.x][start.y] = {}; board.tiles[start.x][start.y].state = ' ';
  board.tiles[end.x][end.y].moved=1;
  return board;
}

//contians main loop
int main(){
  std::string isallowed;
  board board = intialise();
  move moveReturn;
  move move;

  bool is_white = true;
  bool moveAccepted = true;
  print(board);

  std::string inputMove;
  bool running = true;
  while(running == true){
    if(moveAccepted){
      std::cout << "please input move player: ";
      if(is_white == true){std::cout << "white" << '\n';}
      else{std::cout << "black" << '\n';}
    }else{std::cout << "please reenter your move" << '\n'; moveAccepted = 1;}
    std::cin >> inputMove;

    if(inputMove.size() == 2){
      move.state = 'p';
      move.x = inputMove.at(0) - 97;
      move.y = inputMove.at(1) - 49;
    }
    else if(inputMove.size() == 3){
      move.state = inputMove.at(0);
      move.x = inputMove.at(1) - 97;
      move.y = inputMove.at(2) - 49;
    }else{std::cout << "input of the wrong lenght" << '\n'; moveAccepted =0;}

    std::cout << move.state << move.x << move.y << '\n';

    if(moveAccepted == 1){
      moveReturn = allowedMove(board, move, is_white);
      if(moveReturn.state == 'a'){
        board = movePiece(board, moveReturn, move);
      }else{moveAccepted=0;}
    }

    board.PGN.push_back(inputMove);
    if(moveAccepted==1){
      if(is_white == 0){is_white = 1;}else{is_white = 0;}
      print(board);
    }
  }
}
