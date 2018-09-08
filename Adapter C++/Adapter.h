//
// Created by davepj007 on 31/08/18.
//

#ifndef ADAPTER_ADAPTER_H
#define ADAPTER_ADAPTER_H

#include <iostream>
#include <string>
#include "IPlayer.h"
#include "OldPlayer.h"

using namespace std;

class Adapter: public IPlayer, private OldPlayer{ //Clase Adapter hereda de las clases IPlayer, OldPlayer
public:
    Adapter(string song, int volume);

    void play();

    int volume;
};


#endif //ADAPTER_ADAPTER_H
