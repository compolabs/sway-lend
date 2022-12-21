import styled from "@emotion/styled";
import React, { useState } from "react";
import { observer } from "mobx-react-lite";
import Balance from "@src/entities/Balance";
import TokenSelect from "@components/TokenInput/TokenSelect";
import TokenSelectModal from "../TokensSelectModal";

interface IProps {
  balances: Balance[];
  assetId: string;
  setAssetId?: (assetId: string) => void;
  decimals: number;
}

const Root = styled.div`
  display: flex;
  flex-direction: column;
`;

const TokenInput: React.FC<IProps> = (props) => {
  const [openModal, setOpenModal] = useState<boolean>(false);
  const selectedAssetBalance = props.balances?.find(
    ({ assetId }) => assetId === props.assetId
  );

  return (
    <Root>
      <TokenSelect
        selectable
        token={props.balances.find(({ assetId }) => assetId === props.assetId)}
        onClick={() => setOpenModal(!openModal)}
        balance={selectedAssetBalance?.formatBalance}
      />
      {props.setAssetId && (
        <TokenSelectModal
          selectedTokenId={props.assetId}
          visible={openModal}
          onSelect={props.setAssetId}
          balances={props.balances}
          onClose={() => setOpenModal(!openModal)}
        />
      )}
    </Root>
  );
};
export default observer(TokenInput);
