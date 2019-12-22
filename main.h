#include <iostream>
#include <ostream>
#include <string>
#include <vector>

struct move{
  int x;
  int y;
  char state = ' ';
};

struct peice{
  int player = 0; //not played = 0 black =1 white =2
  bool moved = 0; //detect first move
  char state = ' '; //peice in question
};

struct board{
  peice tiles[8][8]; //array of board peices
  std::vector<char> blackLost; //black peices lost
  std::vector<char> whiteLost; //white peices lost
  std::vector<std::string> PGN; //game history
};


namespace Colour{
  enum Code{
    FG_BLUE =34,
    FG_DEFAULT = 39,
    BG_DEFAULT = 49
  };
  class Modifier{
    Code code;
  public:
    Modifier(Code pCode) : code(pCode) {}
    friend std::ostream&
    operator<<(std::ostream& os, const Modifier& mod)  {
      return os << "\033[" << mod.code << "m";
    }
  };
}


int main();
void print(board board);
move allowedMove(board board, move move, bool isWhite);
board intialise();
