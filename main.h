#include <iostream>
#include <ostream>

  struct peice{
    int player = 0 ;
    char state = ' ';
  };


  struct board{
    peice tiles[8][8];
  };
  int main();


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
