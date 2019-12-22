#include "main.h"

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

std::string allowedMove(board board, std::string move, bool isWhite){
  //moving pawns
  int x;
  int y;
  char peice;
  std::string output = "  "; //outputs input location if move not allowed and peice to move if allowed

  int state;
  if(isWhite == 0){
    state =1;
  }else{state=2;}

  if(move.size() ==2){
    peice = 'p';
    x = move.at(0) -97;
    y = move.at(1) -49;


    //checks to see if there's the same colour as the players
  }
  if(move.size() == 3){
    peice = move.at(0);
    switch (peice) {
      case 'r':
      case 'n':
      case 'b':
      case 'q':
      case 'k':
      break;
      default:
      std::cout << "please input r, n, b, q, k for peice to be moved" << '\n';
      return move;
    }
    x = move.at(1) -97;
    y = move.at(2) -49;
    //board.tiles[x-1][y-1].state;
  }

  if(x < 0 || x > 7 || y < 0 || y > 7){
    return move;
  }

  if(state == board.tiles[x][y].player){
    return move;
  }

  switch (peice) {
    case 'p':
      if(isWhite == 1){
        if(board.tiles[x][y+1].state == 'p'){
          board.tiles[x][y] = board.tiles[x][y+1];
          output.at(0) = x+97;
          output.at(1) = (y+1)+49;

          std::cout << x +97;
        }
      }
    }
    std::cout << output << '\n';
    return output;
}

board movePiece(board board, std::string start, std::string end){
  int xb = start.at(0)-97;
  int yb = start.at(1)-49;
  int xn = end.at(0)-97;
  int yn = end.at(1)-49;
  std::cout << xb << yb << xn << yn << '\n';

  board.tiles[xn][yn] = board.tiles[xb][yb];
  board.tiles[xb][yb] = {}; board.tiles[xb][yb].state = ' ';
  return board;
}

int main(){
  std::string isallowed;
  board board = intialise();
  bool is_white = true;
  bool moveAccepted = true;
  print(board);

  std::string move;

  bool running = true;
  while(running == true){
    if(moveAccepted){
      std::cout << "please input move player: ";
      if(is_white == true){std::cout << "white" << '\n';}
      else{std::cout << "black" << '\n';}
    }else{std::cout << "please reenter your move" << '\n'; }
    std::cin >> move;
    if(move.size() < 3 && move.size() > 1){
      isallowed = allowedMove(board, move, is_white);
      if(isallowed == move){moveAccepted=0;}else{moveAccepted=1;
        std::cout << "swap" << isallowed <<'\n';
        board = movePiece(board, isallowed, move);
      }
    }else{moveAccepted = false;}
    board.PGN.push_back(move);
    if(moveAccepted==1){
      if(is_white == 0){is_white = 1;}else{is_white = 0;}
      print(board);
    }
  }
}
