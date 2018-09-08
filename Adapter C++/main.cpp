#include <iostream>
#include <string>

#include "IPlayer.h"
#include "Adapter.h"

using namespace std;

int main() {
    string songName;
    cout << "Enter the name of the Song: " << endl;
    cin >> songName;

    IPlayer* newStuffDad = new Adapter(songName, 50);
    newStuffDad -> play();
    delete newStuffDad;
    return 1;
}